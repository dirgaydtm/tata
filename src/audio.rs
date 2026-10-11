const OREO_OGG: &[u8] = include_bytes!("../assets/audio/oreo.ogg");

fn load_slices() -> Vec<(f64, f64)> {
    #[derive(serde::Deserialize)]
    struct Config {
        defines: std::collections::HashMap<String, Option<[f64; 2]>>,
    }

    serde_json::from_str::<Config>(include_str!("../assets/audio/config.json"))
        .map(|c| {
            c.defines
                .into_values()
                .flatten()
                .map(|[start, dur]| (start / 1000.0, dur / 1000.0))
                .collect()
        })
        .unwrap_or_default()
}

#[cfg(target_arch = "wasm32")]
pub fn play_click() {
    use std::cell::RefCell;
    use web_sys::wasm_bindgen::{JsCast, JsValue, closure::Closure};

    thread_local! {
        static CTX: RefCell<Option<web_sys::AudioContext>> = const { RefCell::new(None) };
        static BUF: RefCell<Option<web_sys::AudioBuffer>> = const { RefCell::new(None) };
        static SLICES: Vec<(f64, f64)> = load_slices();
    }

    CTX.with(|ctx_cell| {
        let mut ctx_opt = ctx_cell.borrow_mut();
        if ctx_opt.is_none()
            && let Ok(ctx) = web_sys::AudioContext::new()
        {
            let js_buf = js_sys::Uint8Array::from(OREO_OGG).buffer();
            if let Ok(promise) = ctx.decode_audio_data(&js_buf) {
                let cb = Closure::once(|val: JsValue| {
                    if let Ok(b) = val.dyn_into() {
                        BUF.with(|buf_cell| *buf_cell.borrow_mut() = Some(b));
                    }
                });
                let _ = promise.then(&cb);
                cb.forget();
            }
            *ctx_opt = Some(ctx);
        }

        let Some(ctx) = ctx_opt.as_ref() else { return };
        if ctx.state() == web_sys::AudioContextState::Suspended {
            let _ = ctx.resume();
        }

        BUF.with(|buf_cell| {
            let buf_ref = buf_cell.borrow();
            let Some(buf) = buf_ref.as_ref() else { return };

            SLICES.with(|slices| {
                let Some(&(start, dur)) = fastrand::choice(slices) else {
                    return;
                };
                let Ok(src) = ctx.create_buffer_source() else {
                    return;
                };
                src.set_buffer(Some(buf));
                let _ = src.connect_with_audio_node(&ctx.destination());
                let _ = src.start_with_when_and_grain_offset_and_grain_duration(0.0, start, dur);
            });
        });
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn play_click() {
    use rodio::{Decoder, OutputStream, Source, buffer::SamplesBuffer};
    use std::{
        io::Cursor,
        sync::{OnceLock, mpsc},
        thread,
    };

    static SENDER: OnceLock<mpsc::SyncSender<()>> = OnceLock::new();

    let sender = SENDER.get_or_init(|| {
        let (tx, rx) = mpsc::sync_channel(16);
        thread::spawn(move || {
            let Ok((_stream, handle)) = OutputStream::try_default() else {
                return;
            };
            let Ok(decoder) = Decoder::new(Cursor::new(OREO_OGG)) else {
                return;
            };

            let channels = decoder.channels();
            let rate = decoder.sample_rate();
            let samples: Vec<f32> = decoder.convert_samples().collect();
            let slices = load_slices();

            while rx.recv().is_ok() {
                if let Some(&(start, dur)) = fastrand::choice(&slices) {
                    let start_idx = (start * rate as f64) as usize * channels as usize;
                    let len = (dur * rate as f64) as usize * channels as usize;
                    let end_idx = (start_idx + len).min(samples.len());

                    if start_idx < samples.len() {
                        let buf = SamplesBuffer::new(channels, rate, &samples[start_idx..end_idx]);
                        let _ = handle.play_raw(buf);
                    }
                }
            }
        });
        tx
    });

    let _ = sender.try_send(());
}
