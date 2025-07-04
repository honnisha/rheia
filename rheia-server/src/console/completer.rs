use common::commands::complitions::{CompleteRequest, CompleteResponse, Completion};
use flume::{Drain, Receiver, Sender};
use lazy_static::lazy_static;
use rustyline::{
    Changeset, Context, Result,
    completion::{Candidate, Completer},
    hint::Hinter,
    line_buffer::LineBuffer,
};
use std::{thread, time::Duration};

lazy_static! {
    static ref CONSOLE_COMPLETE_REQUESTS: (Sender<CompleteRequest>, Receiver<CompleteRequest>) = flume::bounded(1);
    static ref CONSOLE_COMPLETE_RESPONSES: (Sender<CompleteResponse>, Receiver<CompleteResponse>) = flume::bounded(1);
}

#[derive(Default)]
pub(crate) struct CustomCompleter {}

impl CustomCompleter {
    pub(crate) fn iter_complere_requests() -> Drain<'static, CompleteRequest> {
        CONSOLE_COMPLETE_REQUESTS.1.drain()
    }

    pub(crate) fn send_complete_request(request: CompleteRequest) {
        CONSOLE_COMPLETE_REQUESTS.0.send(request).unwrap();
    }

    pub(crate) fn send_complete_response(response: CompleteResponse) {
        CONSOLE_COMPLETE_RESPONSES.0.send(response).unwrap();
    }
}

impl Completer for CustomCompleter {
    type Candidate = CustomCandidate;

    fn complete(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Result<(usize, Vec<CustomCandidate>)> {
        // log::info!("CustomCompleter::complete line:\"{}\" pos:{}", line, pos);
        let request = CompleteRequest::create(line.to_string(), pos);
        CustomCompleter::send_complete_request(request);

        let mut reuslt: Vec<Completion>;
        'waiting: loop {
            for response in CONSOLE_COMPLETE_RESPONSES.1.drain() {
                reuslt = response.get_completions().clone();
                break 'waiting;
            }
            thread::sleep(Duration::from_millis(1));
        }
        let reuslt = reuslt
            .drain(..)
            .map(|c| CustomCandidate::new(c.get_completion().clone()))
            .collect::<Vec<_>>();
        Ok((pos, reuslt))
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

    fn hint(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Option<String> {
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
