// #![allow(dead_code)]
// mod ch6_ring {

//   #[cfg(test)]
//   mod tests {

//     use std::{marker::PhantomData, mem::transmute};

//     /* ************** Write an Error type with anyhow and thiserror ************** */
//     use anyhow::Result;
//     use thiserror::Error;

//     #[derive(Error, Debug, PartialEq)]
//     pub enum InputError {
//       #[error("First letter must be lowercase but was {0:?}")]
//       WrongCase(String),
//       #[error("Invalid character {:?} at index {:?}", .chars, .pos)]
//       InvalidChar { chars: String, pos: usize },
//     }

//     fn check_word(word: &str) -> Result<usize, InputError> {
//       if word.chars().any(|c| !c.is_alphabetic()) {
//         return Err(InputError::InvalidChar {
//           chars: word.chars().filter(|c| !c.is_alphabetic()).collect(),
//           pos: word.chars().position(|c| !c.is_alphabetic()).unwrap(),
//         });
//       }

//       let first_char = word.chars().next().unwrap();
//       if first_char.is_uppercase() {
//         return Err(InputError::WrongCase(first_char.to_string()));
//       }

//       Ok(word.chars().count())
//     }
//     #[test]
//     fn ch5_custom_error() {
//       assert_eq!(check_word("Hello"), Err(InputError::WrongCase("H".to_string())));
//       assert_eq!(
//         check_word("1hello"),
//         Err(InputError::InvalidChar { chars: "1".to_string(), pos: 0 })
//       );
//       assert_eq!(check_word("hello"), Ok(5));
//     }

//     /* ************** Implement a type-level program using PhantomData ************** */
//     // Rock Paper Scissors featuring a commit reveal scheme

//     #[derive(Debug)]
//     struct Player {
//       name: String,
//     }
//     #[repr(transparent)]
//     #[derive(Debug)]
//     struct Game<T, State> {
//       context: Box<T>,
//       _state: PhantomData<State>,
//     }

//     // todo: commit reveal scheme
//     // todo: force withdraw after timeout
//     #[derive(Debug)]
//     struct Start;

//     #[derive(Debug)]
//     struct Join;

//     #[derive(Debug)]
//     struct End;

//     // The state machine is parameterized by the state

//     enum Move {
//       Rock,
//       Paper,
//       Scissor,
//     }
//     impl<T> Game<T, Start> {
//       // start
//       pub fn start(t: T, move: Move) -> Game<T, Start> {
//         Game { context: Box::new(t), _state: PhantomData }
//       }
//     }

//     impl<T> Game<T, Start> {
//       // join
//       fn reply(mut self) -> Game<T, Reply> {
//         self.context = Box::new(*self.context);
//         unsafe { transmute(self) }
//       }
//     }

//     impl<T> Game<T, Reply> {
//       // join
//       fn end(mut self) -> Game<T, End> {
//         self.context = Box::new(*self.context);
//         unsafe { transmute(self) }
//       }
//     }
//     // state: "submitted => pending => sucesfull" or "submitted => pending => failed"

//     // see: https://uncloak.org/courses/rust+cryptography+engineering/course-2022-12-30+Session+5+Notes

//     #[test]
//     fn channel_test() {
//       println!("channel_test");
//       let (alice, _bob) =
//         (Player { name: "Alice".to_string() }, Player { name: "Bob".to_string() });
//       let game = Game::start(alice);
//       println!("game: {:?}", game);
//       let game = game.reply(_bob);
//       println!("game: {:?}", game);

//       // and so on
//     }
//   }
// }
