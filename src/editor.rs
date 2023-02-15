use crate::MidimonParams;
use crate::TimeNotePair;
use chrono::offset::Local;
use chrono::DateTime;
use nih_plug::log::{log, Level};
use nih_plug::prelude::{Editor, NoteEvent};
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::vizia::prelude::{FamilyOwned, Model};
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use rtrb::Consumer;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

// In the editor, we want our MIDI list to update on possibly every process block, reflecting the MIDI we've monitored so far.

mod message_list;
mod piano_roll;

#[derive(Lens, Clone)]
pub(crate) struct EditorData {
    pub(crate) params: Arc<MidimonParams>,
    pub(crate) midi_history: Vec<(SystemTime, NoteEvent<()>)>,
    pub(crate) midi_consumer: Arc<Mutex<Consumer<TimeNotePair>>>,
}

impl Model for EditorData {}

pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::from_size(400, 400)
}

pub(crate) fn create(
    mut editor_data: EditorData,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        assets::register_noto_sans_light(cx);
        assets::register_noto_sans_thin(cx);

        let mut buf = editor_data.midi_consumer.lock().unwrap();
        // let num_free = buf.slots();
        // let buf = buf.read_chunk(crate::MIDI_BUFFER_SIZE - num_free);

        let mut notes: Vec<TimeNotePair> = vec![];

        // for time_note_pair in buf.iter() {}

        while let Ok(time_note_pair) = buf.pop() {
            log!(Level::Info, "Popped note from UI thread");
            notes.push(time_note_pair);
        }

        EditorData {
            params: editor_data.params.clone(),
            midi_history: editor_data
                .midi_history
                .iter()
                .cloned()
                .chain(notes.into_iter())
                .collect(),
            midi_consumer: editor_data.midi_consumer.clone(),
        }
        .build(cx);
        ResizeHandle::new(cx);

        // Layout is defined below
        VStack::new(cx, |cx| {
            log!(Level::Info, "Drawing a VStack");
            Label::new(cx, "Midimon 0.2")
                .font_family(vec![FamilyOwned::Name(String::from(
                    assets::NOTO_SANS_THIN,
                ))])
                .font_size(30.0)
                .height(Pixels(50.0));

            ScrollView::new(cx, 0., 0., false, true, |cx| {
                log!(Level::Info, "Drawing a ScrollView");
                List::new(cx, EditorData::midi_history, move |cx, _index, entry| {
                    log!(Level::Info, "Drawing a list item");
                    let (time, message) = entry.get(cx);
                    let formatted_time = DateTime::<Local>::from(time).format("%r");

                    // Draw different labels for different messages
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
            })
            .size(Pixels(380.));
        });
    })
}
