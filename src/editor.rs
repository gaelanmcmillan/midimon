use crate::MidimonParams;
use chrono::offset::Local;
use chrono::DateTime;
use nih_plug::prelude::{Editor, NoteEvent};
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::vizia::prelude::{FamilyOwned, Model};
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use rtrb::Consumer;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

use crate::TimeNotePair;

// In the editor, we want our MIDI list to update on possibly every process block, reflecting the MIDI we've monitored so far.

#[derive(Lens, Clone)]
pub(crate) struct EditorData {
    pub(crate) params: Arc<MidimonParams>,
    pub(crate) midi_history: Vec<(SystemTime, NoteEvent<()>)>,
    pub(crate) midi_consumer: Arc<Mutex<Consumer<TimeNotePair>>>,
}

impl Model for EditorData {}

pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::from_size(400, 200)
}

pub(crate) fn create(
    editor_data: EditorData,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        assets::register_noto_sans_light(cx);
        assets::register_noto_sans_thin(cx);

        editor_data.clone().build(cx);
        ResizeHandle::new(cx);

        // Layout is defined below
        VStack::new(cx, |cx| {
            Label::new(cx, "Midimon 0.2")
                .font_family(vec![FamilyOwned::Name(String::from(
                    assets::NOTO_SANS_THIN,
                ))])
                .font_size(30.0)
                .height(Pixels(50.0));

            List::new(cx, EditorData::midi_history, move |cx, _index, entry| {
                let (time, message) = entry.get(cx);
                let formatted_time = DateTime::<Local>::from(time).format("%r");

                match message {
                    NoteEvent::NoteOn {
                        timing,
                        voice_id: _,
                        channel,
                        note,
                        velocity,
                    } => Label::new(
                        cx,
                        &format!(
                            "[{}] Note On: {} {} {} {}",
                            formatted_time, timing, channel, note, velocity
                        ),
                    ),
                    NoteEvent::NoteOff {
                        timing,
                        voice_id: _,
                        channel,
                        note,
                        velocity,
                    } => Label::new(
                        cx,
                        &format!(
                            "[{}] Note Off: {} {} {} {}",
                            formatted_time, timing, channel, note, velocity
                        ),
                    ),
                    _ => Label::new(cx, "Unrecognized Message Type"),
                };
            });
        });
    })
}
