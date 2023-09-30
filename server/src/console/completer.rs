use flume::{Drain, Receiver, Sender};
use lazy_static::lazy_static;
use rustyline::{
    completion::{Candidate, Completer},
    hint::Hinter,
    line_buffer::LineBuffer,
    Changeset, Context, Result,
};

pub(crate) struct CompleteRequest {
    pub(crate) line: String,
    pub(crate) pos: usize,
}

lazy_static! {
    static ref CONSOLE_COMPLETE_REQUESTS: (Sender<CompleteRequest>, Receiver<CompleteRequest>) = flume::unbounded();
}

#[derive(Default)]
pub(crate) struct CustomCompleter {}

impl CustomCompleter {
    pub(crate) fn iter_complere_requests() -> Drain<'static, CompleteRequest> {
        CONSOLE_COMPLETE_REQUESTS.1.drain()
    }

    pub(crate) fn send_complere_request(request: CompleteRequest) {
        CONSOLE_COMPLETE_REQUESTS.0.send(request).unwrap();
    }
}

impl Completer for CustomCompleter {
    type Candidate = CustomCandidate;

    fn complete(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Result<(usize, Vec<CustomCandidate>)> {
        let request = CompleteRequest {
            line: line.to_string(),
            pos,
        };
        CustomCompleter::send_complere_request(request);
        let mut reuslts = Vec::new();
        let c = CustomCandidate::new("display1".to_string());
        reuslts.push(c);
        let c = CustomCandidate::new("display2".to_string());
        reuslts.push(c);
        Ok((pos, reuslts))
    }

    /// Updates the edited `line` with the `elected` candidate.
    fn update(&self, line: &mut LineBuffer, start: usize, elected: &str, cl: &mut Changeset) {
        let end = line.pos();
        if line.is_empty() || end < line.len() {
            return;
        }
        line.replace(start..end, elected, cl);
    }
}

#[derive(Clone)]
pub(crate) struct CustomCandidate {
    /// Text to display when listing alternatives.
    pub display: String,
    /// Text to insert in line.
    pub replacement: String,
}

impl CustomCandidate {
    fn new(candidate: String) -> Self {
        Self {
            display: candidate.clone(),
            replacement: candidate,
        }
    }
}

impl Candidate for CustomCandidate {
    fn display(&self) -> &str {
        self.display.as_str()
    }

    fn replacement(&self) -> &str {
        self.replacement.as_str()
    }
}

#[derive(Default)]
pub(crate) struct CustomHinter {}

impl Hinter for CustomHinter {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
        if line.is_empty() || pos < line.len() {
            return None;
        }
        // let start = if ctx.history_index() == ctx.history().len() {
        //     ctx.history_index().saturating_sub(1)
        // } else {
        //     ctx.history_index()
        // };
        // if let Some(sr) = ctx
        //     .history()
        //     .starts_with(line, start, SearchDirection::Reverse)
        //     .unwrap_or(None)
        // {
        //     if sr.entry == line {
        //         return None;
        //     }
        //     return Some(sr.entry[pos..].to_owned());
        // }
        None
    }
}
