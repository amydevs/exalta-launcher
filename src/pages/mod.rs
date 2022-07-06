pub mod config;
pub mod login;
pub mod play;

pub enum Route {
    Play,
    Login,
    Config,
}

#[derive(Clone, Debug)]
pub struct HistoryVec<T> {
    pub history: Vec<T>,
    pub current: T,
}

impl<T> HistoryVec<T> {
    pub fn new(val: T) -> Self {
        Self {
            history: Vec::new(),
            current: val,
        }
    }

    pub fn get(&self) -> &T {
        &self.current
    }

    pub fn set(&mut self, val: T) {
        self.history.push(std::mem::replace(&mut self.current, val));
    }

    pub fn history(&self) -> (&[T], &T) {
        (&self.history, &self.current)
    }

    pub fn revert(&mut self) -> Option<T> {
        self.history
            .pop()
            .map(|val| std::mem::replace(&mut self.current, val))
    }
}
