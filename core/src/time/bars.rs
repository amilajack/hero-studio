use crate::time::{
  Signature,
  TicksTime,
  ticks::TICKS_RESOLUTION
};

pub struct BarsTime {
  bars: u16,
  beats: u16,
  sixteenths: u16,
  ticks: u16
}

impl BarsTime {
  pub fn new(bars: u16, beats: u16, sixteenths: u16, ticks: u16) -> BarsTime {
    BarsTime {
      bars,
      beats,
      sixteenths,
      ticks,
    }
  }

  pub fn from_ticks_time(ticks_time: TicksTime, signature: Signature) -> BarsTime {
    let total_sixteenths = ticks_time.get_ticks() / TICKS_RESOLUTION;
    let num_sixteenths_per_beat = 16 / signature.get_note_value() as u64;
    let total_beats = total_sixteenths / num_sixteenths_per_beat;
    BarsTime {
      bars: (total_beats / signature.get_num_beats() as u64) as u16,
      beats: (total_beats % signature.get_num_beats() as u64) as u16,
      sixteenths: (total_sixteenths % num_sixteenths_per_beat) as u16,
      ticks: (ticks_time.get_ticks() % TICKS_RESOLUTION) as u16,
    }
  }

  pub fn get_bars(&self) -> u16 {
    self.bars
  }

  pub fn get_beats(&self) -> u16 {
    self.beats
  }

  pub fn get_sixteenths(&self) -> u16 {
    self.sixteenths
  }

  pub fn get_ticks(&self) -> u16 {
    self.ticks
  }

  pub fn to_ticks_time(&self, signature: Signature) -> TicksTime {
    let num_sixteenths_per_beat = 16 / signature.get_note_value() as u64;
    let num_ticks_per_beat = num_sixteenths_per_beat * TICKS_RESOLUTION;
    let num_ticks_per_bar = signature.get_num_beats() as u64 * num_ticks_per_beat;
    TicksTime::new(
      self.bars as u64 * num_ticks_per_bar
        + self.beats as u64 * num_ticks_per_beat
        + self.sixteenths as u64 * TICKS_RESOLUTION
        + self.ticks as u64,
    )
  }
}

#[cfg(test)]
mod test {

  use super::BarsTime;
  use crate::time::{
    ticks::TicksTime,
    ticks::TICKS_RESOLUTION,
    Signature,
  };

  #[test]
  pub fn new() {
    let time = BarsTime::new(10, 1, 2, 100);
    assert_eq!(time.get_bars(), 10);
    assert_eq!(time.get_beats(), 1);
    assert_eq!(time.get_sixteenths(), 2);
    assert_eq!(time.get_ticks(), 100);
  }

  #[test]
  pub fn from_ticks_time() {
    let ticks = TicksTime::new(
      TICKS_RESOLUTION * 4 * 3 * 10 + // 10 bars
          TICKS_RESOLUTION * 4 * 2 +  // 2 beats
          TICKS_RESOLUTION     +      // 1 sixteens
          30                          // 30 ticks
    );

    let time = BarsTime::from_ticks_time(ticks, Signature::new(3, 4));
    assert_eq!(time.get_bars(), 10);
    assert_eq!(time.get_beats(), 2);
    assert_eq!(time.get_sixteenths(), 1);
    assert_eq!(time.get_ticks(), 30);
  }

  #[test]
  pub fn to_ticks_time() {
    let signature = Signature::new(3, 4);
    let ticks = TicksTime::new(123456789);
    let time = BarsTime::from_ticks_time(ticks, signature);
    let ticks = time.to_ticks_time(signature);
    assert_eq!(ticks.get_ticks(), 123456789);
  }
}