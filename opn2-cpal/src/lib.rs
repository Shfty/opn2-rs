//! CPAL integration for `opn2`

use std::{
    error::Error,
    sync::{Arc, RwLock},
};

use cpal::{traits::DeviceTrait, traits::HostTrait, Stream, StreamError};

use opn2_driver::Opn2Driver;
use opn2_trait::Opn2Trait;

pub fn opn2_stream<T>(opn2: Arc<RwLock<Opn2Driver<T>>>) -> Result<Stream, Box<dyn Error>>
where
    T: 'static + Opn2Trait + Default + Send + Sync,
{
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("No output device available");

    let supported_configs_range = device.supported_output_configs()?;

    let config = supported_configs_range
        .into_iter()
        .find(|config| config.channels() >= 2)
        .expect("No supported config")
        .with_max_sample_rate();

    opn2.write()
        .unwrap()
        .set_sample_rate(config.sample_rate().0);

    let channels = config.channels() as usize;
    let stream = device.build_output_stream(
        &config.into(),
        data_callback(opn2, channels),
        error_callback(),
        None
    )?;

    Ok(stream)
}

fn data_callback<T>(
    opn2: Arc<RwLock<Opn2Driver<T>>>,
    channels: usize,
) -> impl Fn(&mut [i16], &cpal::OutputCallbackInfo)
where
    T: Opn2Trait + Default,
{
    move |data: &mut [i16], _: &cpal::OutputCallbackInfo| {
        let sample_count = data.chunks(channels).len();
        let mut opn2_guard = opn2.write().unwrap();

        let iter = data
            .chunks_mut(channels)
            .zip(opn2_guard.samples(sample_count));

        for (sample, (l, r)) in iter {
            sample[0] = l;
            sample[1] = r;
        }
    }
}

fn error_callback() -> impl Fn(StreamError) {
    move |err: StreamError| panic!("{}", err)
}
