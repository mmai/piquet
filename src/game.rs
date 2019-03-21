mod cards;

use crate::cards::*;
use std::fmt;

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

#[derive (Debug, PartialEq, Eq, Serialize, Deserialize)]
enum Move { P1Move(PlayerMove), P2Move(PlayerMove) }

#[derive (Debug, PartialEq, Eq, Serialize, Deserialize)]
enum DeclarationResponse { Good, NotGood, Equals } 

#[derive (Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Declaration(Combination)

#[derive (Debug, PartialEq, Eq, Serialize, Deserialize)]
enum PlayerMove { CarteBlanche 
                , CarteRouge   
                , Exchange(Hand) 
                , DeclarationCount(CombinationType, Int))
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

fn movePoints(move: &PlayerMove) -> usize {
    match (move) {
                    CarteBlanche     => 10,
                    CarteRouge       => 20,
                    Pique            => 30,
                    Repique          => 60,
                    WinCards         => 10,
                    Capot            => 40,
                    PlayFirst _      => 1,
                    WinAsSecond      => 1,
                    WinLastTrick     => 1,
                    Declaration(comb) => &comb.points(),
                    _                => 0
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
    pub new(name: String) -> Self {
        Player { hand = noCards
               , isElder = False
               , leftUntilCarteRouge = noCards
               , cardPlayed = None
               , pointCandidate = None
               , sequenceCandidate = None
               , setCandidate = None
               , dealPoints = 0
               , dealWons = 0
               , gamePoints = 0
               , points = 0
               , name
               , sockHandle = stderr
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} : {} rougeLeft={} : ", self.name, self.dealPoints, self.leftUntilCarteRouge.len(), self.hand) 
    }
}

#[derive (Debug, PartialEq, Eq)]
enum DeclarationWinner { Elder, Younger, Tie, Nobody }

struct Game { 
          dealNum             : Deal
        , dealMoves           : Vec<(Move, Int)>
        , deals               : Vec<(Deal, Vec(Move, Int))>
        , deck                : Deck
        , visible             : Deck
        , step                : Step
        , player1             : Player
        , player2             : Player
        , player1SendPortId   : Maybe SendPortId
        , player2SendPortId   : Maybe SendPortId
        , isElderToPlay       : bool
        , pointWinner         : DeclarationWinner
        , pointCombination    : Option<Combination>
        , sequenceWinner      : DeclarationWinner
        , sequenceCombination : Option<Combination>
        , setWinner           : DeclarationWinner
        , setCombination      : Option<Combination>
}

impl Game {
    pub fn new(rng: Rng ) -> Self {
        let deck = Deck::new();
        deck.shuffle(rng);
        Game { dealNum = One
            , dealMoves = []
            , deals = []
            , deck
            , visible = fromList []
            , step = Start
            , player1 = Player::new("Roméo")
            , player2 = Player::new("Juliette")
            , player1SendPortId = None
            , player2SendPortId = None
            , isElderToPlay = True
            , pointWinner = Nobody
            , pointCombination = None
            , sequenceWinner = Nobody
            , sequenceCombination = None
            , setWinner = Nobody
            , setCombination = None
        }
    }
}

