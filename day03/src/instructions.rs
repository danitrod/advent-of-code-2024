#[derive(Debug, PartialEq)]
enum MulInstructionState {
    M,
    U,
    L,
    OpenParenthesis,
    FirstNumber(String),
    Comma,
    SecondNumber(String),
    CloseParenthesis,
    Other,
}

#[derive(Debug, PartialEq)]
enum DoInstructionState {
    D,
    O,
    OpenParenthesis,
    CloseParenthesis,
    Other,
}

#[derive(Debug, PartialEq)]
enum DontInstructionState {
    D,
    O,
    N,
    Apostrophe,
    T,
    OpenParenthesis,
    CloseParenthesis,
    Other,
}

pub struct Mul {
    instruction: String,
    num1: Option<isize>,
    num2: Option<isize>,
    state: MulInstructionState,
}

impl Mul {
    pub fn new() -> Mul {
        Mul {
            instruction: String::new(),
            num1: None,
            num2: None,
            state: MulInstructionState::Other,
        }
    }

    pub fn add_char(&mut self, c: char) {
        match c {
            'm' => {
                self.clear();
                self.state = MulInstructionState::M;
            }
            'u' => {
                if self.state != MulInstructionState::M {
                    self.clear();
                    return;
                }

                self.state = MulInstructionState::U;
            }
            'l' => {
                if self.state != MulInstructionState::U {
                    self.clear();
                    return;
                }

                self.state = MulInstructionState::L;
            }
            '(' => {
                if self.state != MulInstructionState::L {
                    self.clear();
                    return;
                }

                self.state = MulInstructionState::OpenParenthesis;
            }
            ',' => {
                if let MulInstructionState::FirstNumber(num) = &self.state {
                    self.num1 = Some(num.parse::<isize>().unwrap());
                } else {
                    self.clear();
                    return;
                }

                self.state = MulInstructionState::Comma;
            }
            ')' => {
                if let MulInstructionState::SecondNumber(num) = &self.state {
                    self.num2 = Some(num.parse::<isize>().unwrap());
                } else {
                    self.clear();
                    return;
                }

                self.state = MulInstructionState::CloseParenthesis;
            }
            '0'..='9' => match &self.state {
                MulInstructionState::OpenParenthesis => {
                    self.state = MulInstructionState::FirstNumber(c.to_string());
                }
                MulInstructionState::FirstNumber(num) => {
                    self.state = MulInstructionState::FirstNumber(format!("{}{}", num, c));
                }
                MulInstructionState::SecondNumber(num) => {
                    self.state = MulInstructionState::SecondNumber(format!("{}{}", num, c));
                }
                MulInstructionState::Comma => {
                    self.state = MulInstructionState::SecondNumber(c.to_string());
                }
                _ => {
                    self.clear();
                    return;
                }
            },
            _ => {
                self.clear();
                return;
            }
        }

        self.instruction.push(c);
    }

    pub fn clear(&mut self) {
        self.instruction.clear();
        self.num1 = None;
        self.num2 = None;
        self.state = MulInstructionState::Other;
    }

    pub fn is_done(&self) -> bool {
        self.state == MulInstructionState::CloseParenthesis
    }

    pub fn evaluate(&self) -> Option<isize> {
        Some(self.num1? * self.num2?)
    }
}

pub struct Do {
    instruction: String,
    state: DoInstructionState,
}

impl Do {
    pub fn new() -> Do {
        Do {
            instruction: String::new(),
            state: DoInstructionState::D,
        }
    }

    pub fn add_char(&mut self, c: char) {
        match c {
            'd' => {
                self.clear();
                self.state = DoInstructionState::D;
            }
            'o' => {
                if self.state != DoInstructionState::D {
                    self.clear();
                    return;
                }

                self.state = DoInstructionState::O;
            }
            '(' => {
                if self.state != DoInstructionState::O {
                    self.clear();
                    return;
                }

                self.state = DoInstructionState::OpenParenthesis;
            }
            ')' => {
                if self.state != DoInstructionState::OpenParenthesis {
                    self.clear();
                    return;
                }

                self.state = DoInstructionState::CloseParenthesis;
            }
            _ => {
                self.clear();
                return;
            }
        }

        self.instruction.push(c);
    }

    pub fn clear(&mut self) {
        self.instruction.clear();
        self.state = DoInstructionState::Other;
    }

    pub fn is_done(&self) -> bool {
        self.state == DoInstructionState::CloseParenthesis
    }
}

pub struct Dont {
    instruction: String,
    state: DontInstructionState,
}

impl Dont {
    pub fn new() -> Dont {
        Dont {
            instruction: String::new(),
            state: DontInstructionState::D,
        }
    }

    pub fn add_char(&mut self, c: char) {
        match c {
            'd' => {
                self.clear();
                self.state = DontInstructionState::D;
            }
            'o' => {
                if self.state != DontInstructionState::D {
                    self.clear();
                    return;
                }

                self.state = DontInstructionState::O;
            }
            'n' => {
                if self.state != DontInstructionState::O {
                    self.clear();
                    return;
                }

                self.state = DontInstructionState::N;
            }
            '\'' => {
                if self.state != DontInstructionState::N {
                    self.clear();
                    return;
                }

                self.state = DontInstructionState::Apostrophe;
            }
            't' => {
                if self.state != DontInstructionState::Apostrophe {
                    self.clear();
                    return;
                }

                self.state = DontInstructionState::T;
            }
            '(' => {
                if self.state != DontInstructionState::T {
                    self.clear();
                    return;
                }

                self.state = DontInstructionState::OpenParenthesis;
            }
            ')' => {
                if self.state != DontInstructionState::OpenParenthesis {
                    self.clear();
                    return;
                }

                self.state = DontInstructionState::CloseParenthesis;
            }
            _ => {
                self.clear();
                return;
            }
        }

        self.instruction.push(c);
    }

    pub fn clear(&mut self) {
        self.instruction.clear();
        self.state = DontInstructionState::Other;
    }

    pub fn is_done(&self) -> bool {
        self.state == DontInstructionState::CloseParenthesis
    }
}
