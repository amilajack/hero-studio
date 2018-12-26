use crate::time::{
  TicksTime,
  BarsTime,
  Tempo,
  Signature,
  SampleRate
};

const SECONDS_PER_MINUTE: f64 = 60.0;

pub struct TicksDriftCorrection {
  ticks_per_sample: f64,
  error_accumulated: f64,
  last_correction: f64,
}

impl TicksDriftCorrection {

  pub fn new(signature: Signature, tempo: Tempo, sample_rate: SampleRate) -> TicksDriftCorrection {
    let ticks_per_beat = f64::from(BarsTime::new(0, 1, 0, 0).to_ticks(signature));
    let ticks_per_sample = ticks_per_beat * f64::from(tempo) / (SECONDS_PER_MINUTE * sample_rate as f64);

    TicksDriftCorrection {
      ticks_per_sample: ticks_per_sample,
      error_accumulated: 0.0,
      last_correction: 0.0
    }
  }

  pub fn get_ticks_per_sample(&self) -> f64 {
    self.ticks_per_sample
  }

  pub fn get_error_accumulated(&self) -> f64 {
    self.error_accumulated
  }

  pub fn get_last_correction(&self) -> f64 {
    self.last_correction
  }

  pub fn next(&mut self, samples: u32) -> TicksTime {
    let ticks = self.ticks_per_sample * samples as f64;
    let ticks_rounded = ticks.round();
    let ticks_error = ticks - ticks_rounded;
    let total_error = self.error_accumulated + ticks_error;
    if total_error.abs() >= 1.0 {
      self.last_correction = total_error.round();
      self.error_accumulated = total_error - self.last_correction;
      TicksTime::new((ticks_rounded + self.last_correction) as u64)
    }
    else {
      self.last_correction = 0.0;
      self.error_accumulated = total_error;
      TicksTime::new(ticks_rounded as u64)
    }
  }
}

#[cfg(test)]
mod test {

  use super::{Signature, Tempo, TicksTime};
  use super::TicksDriftCorrection;

  #[test]
  pub fn ticks_drift_correction_new() {
    let correction = TicksDriftCorrection::new(Signature::new(4, 4), Tempo::new(60), 44100);
    assert_eq!(correction.ticks_per_sample, 0.08707482993197278);
    assert_eq!(correction.error_accumulated, 0.0);
    assert_eq!(correction.last_correction, 0.0);
  }

  #[test]
  pub fn ticks_drift_correction_next() {
    let mut correction = TicksDriftCorrection::new(Signature::new(4, 4), Tempo::new(60), 44100);
    let ticks = correction.next(100);
    assert_eq!(ticks, TicksTime::new(9));
    let ticks = correction.next(100);
    assert_eq!(ticks, TicksTime::new(9));
    let ticks = correction.next(100);
    assert_eq!(ticks, TicksTime::new(9));
    let ticks = correction.next(100);
    assert_eq!(ticks, TicksTime::new(8));
    let ticks = correction.next(100);
    assert_eq!(ticks, TicksTime::new(9));
  }
}
