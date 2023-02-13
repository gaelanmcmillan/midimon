use chrono::offset::Local;
use chrono::DateTime;
use nih_plug::prelude::{Editor, NoteEvent};
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::vizia::prelude::{FamilyOwned, Model};
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use crate::MidimonParams;

// In the editor, we want our MIDI list to update on possibly every process block, reflecting the MIDI we've monitored so far.

#[derive(Lens)]
struct Data {
    params: Arc<MidimonParams>,
    midi_events: Vec<(SystemTime, NoteEvent<()>)>,
}

impl Model for Data {}

pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::from_size(400, 200)
}

pub(crate) fn create(
    params: Arc<MidimonParams>,
    editor_state: Arc<ViziaState>,
    _midi_events: Vec<(SystemTime, NoteEvent<()>)>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        assets::register_noto_sans_light(cx);
        assets::register_noto_sans_thin(cx);

        Data {
            params: params.clone(),
            midi_events: vec![
                (
                    SystemTime::now(),
                    NoteEvent::NoteOn {
                        timing: 3,
                        voice_id: None,
                        channel: 0,
                        note: 60,
                        velocity: 0.5,
                    },
                ),
                (
                    SystemTime::now() + Duration::from_secs(4),
                    NoteEvent::NoteOff {
                        timing: 87,
                        voice_id: None,
                        channel: 0,
                        note: 60,
                        velocity: 0.,
                    },
                ),
            ], //midi_events.clone(),
        }
        .build(cx);

        ResizeHandle::new(cx);

        // Layout is defined below
        VStack::new(cx, |cx| {
            Label::new(cx, "Midimon")
                .font_family(vec![FamilyOwned::Name(String::from(
                    assets::NOTO_SANS_THIN,
                ))])
                .font_size(30.0)
                .height(Pixels(50.0));

            List::new(cx, Data::midi_events, move |cx, _index, entry| {
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
