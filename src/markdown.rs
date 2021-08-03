use std::fs::{self, read_dir};

fn main() {
  println!(
    "| {: <25} | {: <15} | {: <20} |",
    "name", "avg. time", "avg. thrpt"
  );

  println!(
    "| {} | {} | {} |",
    "-".repeat(25),
    "-".repeat(15),
    "-".repeat(20)
  );

  ["10k", "100k"]
    .iter()
    .map(|size_name| {
      let size = size_name.replace("k", "");
      let size = size.parse::<f64>().unwrap() * 1000.0;
      let mut tests = read_dir(format!("target/criterion/compete_{}", size_name))
        .unwrap()
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_dir())
        .filter(|entry| !entry.file_name().to_string_lossy().starts_with("report"))
        .map(|entry| {
          let name = entry.file_name();
          let name = name.to_string_lossy();
          let path = entry.path().join("new").join("estimates.json");
          let json = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Failed to read `{:?}`", &path));

          let index = json
            .find("point_estimate")
            .unwrap_or_else(|| panic!("No `point_estimates` in `{:?}`", &path));

          let mut time = String::with_capacity(255);
          let mut in_time = false;

          for ch in json.chars().skip(index) {
            if ch.is_digit(10) || ch == '.' {
              time.push(ch);
              in_time = true;
            } else if in_time {
              break;
            }
          }

          let time = time.parse::<f64>().unwrap();
          let second = time * 1e-9;

          let throughput: f64 = size / second;

          let name = name.to_string();

          (name, time, throughput)
        })
        .collect::<Vec<_>>();

      tests.sort_by(|a, b| a.2.clone().partial_cmp(&b.2).unwrap());
      tests.reverse();
      tests
        .iter()
        .map(|entry| {
          let mut name = format!("{}/{}", size_name, entry.0);
          let mut time = to_pretty_time(entry.1);
          let mut throughput = to_pretty_throughput(entry.2);

          if entry.0 == "simd-adler32" {
            name = format!("**{}**", name);
            time = format!("**{}**", time);
            throughput = format!("**{}**", throughput);
          }

          println!("| {: <25} | {: <15} | {: <20} |", name, time, throughput)
        })
        .collect::<Vec<_>>()
    })
    .enumerate()
    .for_each(|(i, _)| {
      if i < 1 {
        println!(
          "| {} | {} | {} |",
          "-".repeat(25),
          "-".repeat(15),
          "-".repeat(20)
        );
      }
    });
}

fn to_pretty_time(time: f64) -> String {
  const US: f64 = 1000.0;
  const MS: f64 = 1e+6;
  const S: f64 = 1e+9;

  match time {
    x if x > S => format!("{:.3} ns", x / S),
    x if x > US => format!("{:.3} µs", x / US),
    x if x > MS => format!("{:.3} ms", x / MS),
    x => format!("{:.3} ns", x),
  }
}

fn to_pretty_throughput(bps: f64) -> String {
  const KB: f64 = 1024.0;
  const MB: f64 = KB * 1024.0;
  const GB: f64 = MB * 1024.0;
  const TB: f64 = GB * 1024.0;

  match bps {
    x if x > TB => format!("{:.3} TiB/s", x / TB),
    x if x > GB => format!("{:.3} GiB/s", x / GB),
    x if x > MB => format!("{:.3} MiB/s", x / MB),
    x if x > KB => format!("{:.3} KiB/s", x / KB),
    x => format!("{:.3} B/s", x),
  }
}
