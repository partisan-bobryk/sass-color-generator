
use angular_units::Deg;
use prisma::Hsl;

pub enum ColorTemperature {
  Warm,
  Cool,
  Neutral
}

pub trait ColorTheory {
  fn get_temperature(&self) -> ColorTemperature;
}

impl ColorTheory for Hsl<f32>{
  fn get_temperature(&self) -> ColorTemperature {
    let hue_deg:Deg<f32> = self.hue();
    let hue: f32 = hue_deg.0;
    match hue {
      x if x >= 0.0 && x < 155.0  => ColorTemperature::Warm,
      x if x >= 155.0 && x < 270.0 => ColorTemperature::Cool,
      _ => ColorTemperature::Neutral
    }

  }
}

