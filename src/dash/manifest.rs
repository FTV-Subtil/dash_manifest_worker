
use mcai_worker_sdk::debug;
use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};

#[derive(Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(
  rename = "MPD",
  prefix = "mpd",
  namespace = "mpd: urn:mpeg:dash:schema:mpd:2011"
)]
pub struct Manifest {
  #[yaserde(rename = "minBufferTime", attribute)]
  min_buffer_time: Option<String>,
  #[yaserde(rename = "mediaPresentationDuration", attribute)]
  media_presentation_duration: String,
  #[yaserde(rename = "maxSegmentDuration", attribute)]
  max_segment_duration: String,
  #[yaserde(attribute)]
  profiles: String,
  #[yaserde(rename = "type", attribute)]
  kind: String,

  #[yaserde(rename = "ProgramInformation")]
  program_information: Option<ProgramInformation>,
  #[yaserde(rename = "Period")]
  period: Period,
}

impl Manifest {
  pub fn add_adaptation_set(&mut self, adaptation_set: AdaptationSet) {
    self.period.adaptation_set.push(adaptation_set);
  }

  pub fn remove_adaptation_set(&mut self, ttml_language: &str, ttml_role: &str) {
    self.period.adaptation_set = self
      .period
      .adaptation_set
      .iter()
      .filter(|adaptation_set| {
        if ttml_language == adaptation_set.language.as_str()
          && adaptation_set.role
            == Some(Role {
              scheme_id_uri: "urn:mpeg:dash:role:2011".to_string(),
              id: None,
              content: Some(ttml_role.to_string()),
            })
        {
          true
        } else {
          false
        }
      })
      .map(|adaptation_set| adaptation_set.clone())
      .collect();
  }

  pub fn prefix_urls(&mut self, reference_url: &str) {
    let reference_url = reference_url.to_string().replace("manifest.mpd", "");

    self.period.adaptation_set = self
      .period
      .adaptation_set
      .iter()
      .map(|adaptation_set| {
        let mut adaptation_set = adaptation_set.clone();
        adaptation_set.representation = adaptation_set
          .representation
          .iter()
          .map(|representation| {
            let mut representation = representation.clone();
            representation.base_url = if !representation.base_url.starts_with("http") {
              reference_url.clone() + &representation.base_url
            } else {
              representation.base_url
            };
            representation
          })
          .collect();
        adaptation_set
      })
      .collect();
  }
}

#[derive(Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "mpd", namespace = "mpd: urn:mpeg:dash:schema:mpd:2011")]
pub struct ProgramInformation {
  #[yaserde(prefix = "mpd", rename = "moreInformationURL", attribute)]
  more_information_url: String,

  #[yaserde(prefix = "mpd", rename = "Title")]
  title: String,
}

impl Default for ProgramInformation {
  fn default() -> Self {
    ProgramInformation {
      more_information_url: "".to_string(),
      title: "".to_string(),
    }
  }
}

#[derive(Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "mpd", namespace = "mpd: urn:mpeg:dash:schema:mpd:2011")]
pub struct Period {
  #[yaserde(attribute)]
  duration: String,
  #[yaserde(rename = "AdaptationSet")]
  adaptation_set: Vec<AdaptationSet>,
}

impl Default for Period {
  fn default() -> Self {
    Period {
      duration: "".to_string(),
      adaptation_set: vec![],
    }
  }
}

#[derive(Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "mpd", namespace = "mpd: urn:mpeg:dash:schema:mpd:2011")]
pub struct AdaptationSet {
  #[yaserde(rename = "segmentAlignment", attribute)]
  segment_alignement: bool,
  #[yaserde(rename = "maxWidth", attribute)]
  max_width: Option<u32>,
  #[yaserde(rename = "maxHeight", attribute)]
  max_height: Option<u32>,
  #[yaserde(rename = "maxFrameRate", attribute)]
  max_frame_rate: Option<u32>,
  #[yaserde(rename = "par", attribute)]
  pixel_aspect_ratio: Option<String>,
  #[yaserde(rename = "lang", attribute)]
  language: String,
  #[yaserde(rename = "subsegmentAlignment", attribute)]
  subsegment_alignment: Option<bool>,
  #[yaserde(rename = "subsegmentStartsWithSAP", attribute)]
  subsegment_starts_sith_sap: Option<String>,
  #[yaserde(rename = "contentType", attribute)]
  content_type: Option<String>,

  #[yaserde(rename = "Role")]
  role: Option<Role>,
  #[yaserde(rename = "Representation")]
  representation: Vec<Representation>,
}

impl AdaptationSet {
  pub fn new_ttml_subtitle(file_path: &str, language: &str, role: &str, file_size: u64) -> Self {
    AdaptationSet {
      segment_alignement: true,
      max_width: None,
      max_height: None,
      max_frame_rate: None,
      pixel_aspect_ratio: None,
      language: language.to_string(),
      subsegment_alignment: None,
      subsegment_starts_sith_sap: None,
      content_type: Some("text".to_string()),
      role: Some(Role {
        scheme_id_uri: "urn:mpeg:dash:role:2011".to_string(),
        id: None,
        content: Some(role.to_string()),
      }),
      representation: vec![Representation {
        id: "s1".to_string(),
        mime_type: Some("application/ttml+xml".to_string()),
        codecs: None,
        width: None,
        height: None,
        frame_rate: None,
        sample_aspect_ratio: None,
        audio_sampling_rate: None,
        start_with_sap: None,
        bandwidth: file_size,
        audio_channel_configuration: vec![],
        base_url: file_path.to_string(),
        segment_base: None,
      }],
    }
  }
}

#[derive(Debug, Clone, PartialEq, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "mpd", namespace = "mpd: urn:mpeg:dash:schema:mpd:2011")]
pub struct Role {
  #[yaserde(rename = "schemeIdUri", attribute)]
  scheme_id_uri: String,
  #[yaserde(rename = "value", attribute)]
  content: Option<String>,
  #[yaserde(attribute)]
  id: Option<String>,
}

impl Default for Role {
  fn default() -> Self {
    Role {
      scheme_id_uri: "".to_string(),
      content: None,
      id: None,
    }
  }
}

#[derive(Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "mpd", namespace = "mpd: urn:mpeg:dash:schema:mpd:2011")]
pub struct Representation {
  #[yaserde(prefix = "mpd", attribute)]
  id: String,
  #[yaserde(prefix = "mpd", rename = "mimeType", attribute)]
  mime_type: Option<String>,
  #[yaserde(prefix = "mpd", attribute)]
  codecs: Option<String>,
  #[yaserde(prefix = "mpd", attribute)]
  width: Option<u32>,
  #[yaserde(prefix = "mpd", attribute)]
  height: Option<u32>,
  #[yaserde(prefix = "mpd", rename = "frameRate", attribute)]
  frame_rate: Option<u32>,
  #[yaserde(prefix = "mpd", rename = "audioSamplingRate", attribute)]
  audio_sampling_rate: Option<String>,
  #[yaserde(prefix = "mpd", rename = "sar", attribute)]
  sample_aspect_ratio: Option<String>,
  #[yaserde(prefix = "mpd", rename = "startWithSAP", attribute)]
  start_with_sap: Option<u8>,
  #[yaserde(prefix = "mpd", attribute)]
  bandwidth: u64,

  #[yaserde(prefix = "mpd", rename = "AudioChannelConfiguration")]
  audio_channel_configuration: Vec<AudioChannelConfiguration>,
  #[yaserde(prefix = "mpd", rename = "BaseURL")]
  base_url: String,
  #[yaserde(prefix = "mpd", rename = "SegmentBase")]
  segment_base: Option<SegmentBase>,
}

impl Default for Representation {
  fn default() -> Self {
    Representation {
      id: "".to_string(),
      mime_type: None,
      codecs: None,
      width: None,
      height: None,
      frame_rate: None,
      sample_aspect_ratio: None,
      audio_sampling_rate: None,
      start_with_sap: None,
      bandwidth: 0,
      base_url: "".to_string(),
      segment_base: None,
      audio_channel_configuration: vec![],
    }
  }
}

#[derive(Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "mpd", namespace = "mpd: urn:mpeg:dash:schema:mpd:2011")]
pub struct SegmentBase {
  #[yaserde(rename = "indexRangeExact", attribute)]
  index_range_exact: bool,
  #[yaserde(rename = "indexRange", attribute)]
  index_range: Option<String>,
  #[yaserde(rename = "presentationTimeOffset", attribute)]
  presentation_time_offset: Option<u64>,
  #[yaserde(rename = "Initialization")]
  initialization: Initialization,
}

impl Default for SegmentBase {
  fn default() -> Self {
    SegmentBase {
      index_range_exact: false,
      index_range: None,
      presentation_time_offset: None,
      initialization: Initialization::default(),
    }
  }
}

#[derive(Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "mpd", namespace = "mpd: urn:mpeg:dash:schema:mpd:2011")]
pub struct Initialization {
  #[yaserde(rename = "sourceURL", attribute)]
  source_url: Option<String>,
  #[yaserde(attribute)]
  range: Option<String>,
}

impl Default for Initialization {
  fn default() -> Self {
    Initialization {
      source_url: None,
      range: None,
    }
  }
}

#[derive(Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "mpd", namespace = "mpd: urn:mpeg:dash:schema:mpd:2011")]
pub struct AudioChannelConfiguration {
  #[yaserde(rename = "schemeIdUri", attribute)]
  scheme_id_uri: String,
  #[yaserde(rename = "value", attribute)]
  content: Option<String>,
  #[yaserde(attribute)]
  id: Option<String>,
}

impl Default for AudioChannelConfiguration {
  fn default() -> Self {
    AudioChannelConfiguration {
      scheme_id_uri: "".to_string(),
      content: None,
      id: None,
    }
  }
}

#[test]
fn sample_1() {
  use std::fs::File;
  use std::io::Read;
  use yaserde::de::from_str;
  use yaserde::ser::to_string;

  let filename = "tests/sample_1.mpd";
  let mut f = File::open(filename).expect("file not found");
  let mut contents = String::new();
  f.read_to_string(&mut contents)
    .expect("something went wrong reading the file");

  let loaded: Result<Manifest, String> = from_str(&contents);

  if let Ok(ref mpd) = loaded {
    let s = to_string(mpd).unwrap();
    println!("{}", s);
  }
}

#[test]
fn sample_2() {
  use std::fs::File;
  use std::io::Read;
  use yaserde::de::from_str;
  use yaserde::ser::to_string;

  let filename = "tests/sample_2.mpd";
  let mut f = File::open(filename).expect("file not found");
  let mut contents = String::new();
  f.read_to_string(&mut contents)
    .expect("something went wrong reading the file");

  let loaded: Result<Manifest, String> = from_str(&contents);
  if let Ok(ref mpd) = loaded {
    let s = to_string(mpd).unwrap();
    println!("{}", s);
  }
}

#[test]
fn sample_3() {
  use std::fs::File;
  use std::io::Read;
  use yaserde::de::from_str;
  use yaserde::ser::to_string;

  let filename = "tests/sample_3.mpd";
  let mut f = File::open(filename).expect("file not found");
  let mut contents = String::new();
  f.read_to_string(&mut contents)
    .expect("something went wrong reading the file");

  let loaded: Result<Manifest, String> = from_str(&contents);

  if let Ok(ref mpd) = loaded {
    let s = to_string(mpd).unwrap();
    println!("{}", s);
  }
}
