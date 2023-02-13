use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;
use std::sync::Arc;
use std::time::SystemTime;

mod editor;

// This project was started with the cookiecutter template for NIH-plug
// Source:
// https://github.com/robbert-vdh/nih-plug-template

struct Midimon {
    params: Arc<MidimonParams>,
    midi_events: Arc<Vec<(SystemTime, NoteEvent<()>)>>,
}

#[derive(Params)]
struct MidimonParams {
    // The parameter's ID is used to identify the parameter in the wrappred plugin API. As long as
    // these IDs remain constant, you can rename and reorder these fields as you wish. The
    // parameters are exposed to the host in the same order they were defined. In this case, this
    // gain parameter is stored as linear gain while the values are displayed in decibels.
    // #[id = "gain"]
    // pub gain: FloatParam,
    #[persist = "editor-state"]
    editor_state: Arc<ViziaState>,
}

impl Default for Midimon {
    fn default() -> Self {
        Self {
            params: Arc::new(MidimonParams::default()),
            midi_events: Arc::new(Vec::new()),
        }
    }
}

impl Default for MidimonParams {
    fn default() -> Self {
        Self {
            // Initialize parameter defaults here.
            editor_state: editor::default_state(),
        }
    }
}

impl Plugin for Midimon {
    const NAME: &'static str = "Midimon";
    const VENDOR: &'static str = "Gaelan McMillan";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "your@email.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const DEFAULT_INPUT_CHANNELS: u32 = 2;
    const DEFAULT_OUTPUT_CHANNELS: u32 = 2;

    const DEFAULT_AUX_INPUTS: Option<AuxiliaryIOConfig> = None;
    const DEFAULT_AUX_OUTPUTS: Option<AuxiliaryIOConfig> = None;

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    // If the plugin can send or receive SysEx messages, it can define a type to wrap around those
    // messages here. The type implements the `SysExMessage` trait, which allows conversion to and
    // from plain byte buffers.
    type SysExMessage = ();
    // More advanced plugins can use this to run expensive background tasks. See the field's
    // documentation for more information. `()` means that the plugin does not have any background
    // tasks.
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    /// Build the Editor window
    fn editor(&self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(
            self.params.clone(),
            self.params.editor_state.clone(),
            self.midi_events.to_vec(),
        )
    }

    fn accepts_bus_config(&self, config: &BusConfig) -> bool {
        // This works with any symmetrical IO layout
        config.num_input_channels == config.num_output_channels && config.num_input_channels > 0
    }

    fn initialize(
        &mut self,
        _bus_config: &BusConfig,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        // Resize buffers and perform other potentially expensive initialization operations here.
        // The `reset()` function is always called right after this function. You can remove this
        // function if you do not need it.
        true
    }

    fn reset(&mut self) {
        // Reset buffers and envelopes here. This can be called from the audio thread and may not
        // allocate. You can remove this function if you do not need it.
    }

    fn process(
        &mut self,
        _buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        while let Some(event) = context.next_event() {
            match event {
                NoteEvent::NoteOn { .. } => (),
                NoteEvent::NoteOff { .. } => (),
                _ => (),
            }
        }
        ProcessStatus::Normal
    }
}

impl ClapPlugin for Midimon {
    const CLAP_ID: &'static str = "com.your-domain.Midimon";
    const CLAP_DESCRIPTION: Option<&'static str> =
        Some("A plugin to monitor incoming midi events.");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    // Don't forget to change these features
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo];
}

impl Vst3Plugin for Midimon {
    const VST3_CLASS_ID: [u8; 16] = *b"m1d1p33kl00ks33s";

    // And don't forget to change these categories, see the docstring on `VST3_SUBCATEGORIES` for more
    // information
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Dynamics];
}

nih_export_clap!(Midimon);
nih_export_vst3!(Midimon);
