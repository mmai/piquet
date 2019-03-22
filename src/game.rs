use crate::cards::*;
use std::fmt;
use serde::{Serialize, Deserialize};
use rand_core::SeedableRng;
use rand::Rng;

use crate::combinations::*;

#[derive (Debug, PartialEq, Eq, Serialize, Deserialize)]
enum Deal { One, Two, Three, Four, Five, Six }

#[derive (Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
enum Step { Start 
          , Deal
          , ExchangeElder
          , ExchangeYounger 
          , DeclarePointElder
          , DeclarePointResponse 
          , SetPointsPointElder 
          , DeclareSequenceElder
          , DeclareSequenceResponse
          , SetPointsSequenceElder 
          , DeclareSetElder
          , DeclareSetResponse
          , SetPointsSetElder 
          , PlayFirstCard
          , SetPointsPointYounger 
          , SetPointsSequenceYounger 
          , SetPointsSetYounger 
          , PlayCards
          , PlayEnd
          , End
}

impl Step {
    pub fn succ(&self) -> Option<Self> {
        use Step::*;
        match self {                 
            Start                    => Some(Deal)
          , Deal                     => Some(ExchangeElder)
          , ExchangeElder            => Some(ExchangeYounger)
          , ExchangeYounger          => Some(DeclarePointElder)
          , DeclarePointElder        => Some(DeclarePointResponse)
          , DeclarePointResponse     => Some(SetPointsPointElder)
          , SetPointsPointElder      => Some(DeclareSequenceElder)
          , DeclareSequenceElder     => Some(DeclareSequenceResponse)
          , DeclareSequenceResponse  => Some(SetPointsSequenceElder)
          , SetPointsSequenceElder   => Some(DeclareSetElder)
          , DeclareSetElder          => Some(DeclareSetResponse)
          , DeclareSetResponse       => Some(SetPointsSetElder)
          , SetPointsSetElder        => Some(PlayFirstCard)
          , PlayFirstCard            => Some(SetPointsPointYounger)
          , SetPointsPointYounger    => Some(SetPointsSequenceYounger)
          , SetPointsSequenceYounger => Some(SetPointsSetYounger)
          , SetPointsSetYounger      => Some(PlayCards)
          , PlayCards                => Some(PlayEnd)
          , PlayEnd                  => Some(End)
          , End                      => None
        }
    }
}

#[derive (Debug, PartialEq, Eq, Serialize, Deserialize)]
enum Move { P1Move(PlayerMove), P2Move(PlayerMove) }

#[derive (Debug, PartialEq, Eq, Serialize, Deserialize)]
enum DeclarationResponse { Good, NotGood, Equals } 

#[derive (Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Declaration(Combination);

#[derive (Debug, PartialEq, Eq, Serialize, Deserialize)]
enum PlayerMove { CarteBlanche 
                , CarteRouge   
                , Exchange(Hand) 
                , DeclarationCount(CombinationType, u32)
                , DeclarationUpper(CombinationType, Rank) 
                , PlayerResponse(CombinationType, DeclarationResponse)
                , Declaration(Combination) 
                , Repique      
                , PlayFirst(Card) 
                , Pique        
                , WinAsSecond  
                , WinLastTrick 
                , PlayCard(Card)
                , WinCards     
                , Capot        
}

impl PlayerMove {
    pub fn movePoints(&self) -> usize {
        use PlayerMove::*;
        match self {
            CarteBlanche     => 10,
            CarteRouge       => 20,
            Pique            => 30,
            Repique          => 60,
            WinCards         => 10,
            Capot            => 40,
            PlayFirst(_)      => 1,
            WinAsSecond      => 1,
            WinLastTrick     => 1,
            Declaration(comb) => comb.points(),
            _                => 0
        }
    }
}


#[derive (Debug, PartialEq, Eq, Serialize, Deserialize)]
enum PiquetError { NotYourTurnError 
                 , InvalidForStepError(Step) 
                 , InvalidCombination
                 , CardNotInHand
                 , AlreadyConnectedError
                 , NotConnectedError
                 , UnknownCommand
}


#[derive (Debug, Serialize, Deserialize)]
struct Player {
          hand: Hand
        , isElder: bool
        , leftUntilCarteRouge: Hand
        , cardPlayed: Option<Card>
        , pointCandidate: Option<Combination>
        , sequenceCandidate: Option<Combination>
        , setCandidate: Option<Combination>
        , dealPoints: u32
        , dealWons: u32
        , gamePoints: u32
        , points: u32
        , name: String
} 

impl Player {
    pub fn new(name: String) -> Self {
        Player { hand: Hand::empty_hand()
               , isElder: false
               , leftUntilCarteRouge: Hand::empty_hand()
               , cardPlayed: None
               , pointCandidate: None
               , sequenceCandidate: None
               , setCandidate: None
               , dealPoints: 0
               , dealWons: 0
               , gamePoints: 0
               , points: 0
               , name
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} : {} rougeLeft={} : {}", self.name, self.dealPoints, self.leftUntilCarteRouge.len(), self.hand) 
    }
}

#[derive (Debug, PartialEq, Eq)]
enum DeclarationWinner { Elder, Younger, Tie, Nobody }

#[derive (Debug)]
pub struct Game { rng: rand_xorshift::XorShiftRng
        , dealNum             : Deal
        , dealMoves           : Vec<(Move, u32)>
        , deals               : Vec<(Deal, Vec<(Move, u32)>)>
        , deck                : Deck
        , visible             : Deck
        , step                : Step
        , player1             : Player
        , player2             : Player
        // , player1SendPortId   : Option<SendPortId>
        // , player2SendPortId   : Option<SendPortId>
        , isElderToPlay       : bool
        , pointWinner         : DeclarationWinner
        , pointCombination    : Option<Combination>
        , sequenceWinner      : DeclarationWinner
        , sequenceCombination : Option<Combination>
        , setWinner           : DeclarationWinner
        , setCombination      : Option<Combination>
}

impl Game {
    pub fn new(seed:[u8; 16]) -> Self {
        let mut deck = Deck::new();
        let mut rng = rand_xorshift::XorShiftRng::from_seed(seed);
        deck.shuffle(&mut rng);
        Game { rng
            , dealNum: Deal::One
            , dealMoves: vec![]
            , deals: vec![]
            , deck
            , visible: Deck::empty_deck()
            , step: Step::Start
            , player1: Player::new("Roméo".to_string())
            , player2: Player::new("Juliette".to_string())
            // , player1SendPortId: None
            // , player2SendPortId: None
            , isElderToPlay: true
            , pointWinner: DeclarationWinner::Nobody
            , pointCombination: None
            , sequenceWinner: DeclarationWinner::Nobody
            , sequenceCombination: None
            , setWinner: DeclarationWinner::Nobody
            , setCombination: None
        }
    }

    pub fn choose_elder(&mut self){
        self.player1.isElder = self.rng.gen();
        self.player2.isElder = !self.player1.isElder;
    }

    pub fn deal<'hands>(&mut self){
        // let hands = self.deck.draw_hands(12, 2);
        let hands = vec![Hand::new(vec![]), Hand::new(vec![])];
        self.player1.hand = hands[0];
        self.player1.leftUntilCarteRouge = self.player1.hand.clone();
        self.player1.pointCandidate = None;
        self.player1.sequenceCandidate = None;
        self.player1.setCandidate = None;

        self.player2.hand = hands[1];
        self.player2.leftUntilCarteRouge = self.player2.hand.clone();
        self.player2.pointCandidate = None;
        self.player2.sequenceCandidate = None;
        self.player2.setCandidate = None;
        self.dealMoves = vec![];
        self.step = Step::Deal.succ().expect("No more step");
    }
}

