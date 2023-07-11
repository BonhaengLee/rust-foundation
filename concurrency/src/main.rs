fn process_files(filenames: Vec<String>, glossary: &GigabyteMap) {
    // for document in filenames {
    //     let text = load(&document)?;
    //     let results = process(text);
    //     save(&document, results)?;
    // }
    for worklist in worklists {
      // 여기서 .clone() 호출은 Arc를 복제하고 레퍼런스 카운트를 올리는 일만 한다.
      // GigabyteMap을 복제하는 게 아니다.
      let glossary_for_child = glossary.clone();
      thread_handles.push(thread::spawn(move || {
        process_files(worklist, &glossary_for_child);
      }));
    }
}

// spawn & join
use std::thread;

thread::spawn(|| {
    println!("hello from a child thread!");
});

// process_files parallel version
use std::{thread, io};

fn process_files_in_parallel(filenames: Vec<String>) -> io::Result<()>{
  // 작업을 몇 개의 덩어리로 분할한다.
  const NTHREADS: usize = 8;
  let worklists = split_vec_into_chunks(filenames, NTHREADS);

  // 포크: 각 덩어리를 처리하는 스레드를 생성한다.
  let mut thread_handles = vec![];
  for worklist in worklists {
    thread_handles.push(thread::spawn(move ||
      process_files(worklist)));
  }

  // 조인: 각 스레드가 종료될 때까지 기다린다.
  for handle in thread_handles {
    handle.join().unwrap()?;
  }

  Ok(())
}

// 망델브로
let mut pixels = vec![0; bounds.0 * bounds.1];

// 이 구간에서 `pixels`를 여러 줄의 띠로 나눈다.
{
  let bands: Vec<(usize, &mut [u8])> =
    pixels.chunks_mut(bounds.0).enumerate().collect();

    bands.into_par_iter().for_each(|(i, band)| {
      let top = i;
      let band_bounds = (bounds.0, 1);
      let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
      let band_lower_right = pixel_to_point(bounds, (bounds.0, top + 1), upper_left, lower_right);

      render(band, band_bounds, band_upper_left, band_lower_right);
    });
}

// write_image(&args[1], &pixels, bounds)
//   .expect("error writing PNG file");

fn run_pipeline(documents: Vec<PathBuf>, output_dir: PathBuf) -> io::Result<()> {
  let (texts, h1) = start_file_reader_thread(documents);
  let(pints h2) = start_file_indexing_thread(texts);
  let(gallons, h3) = start_in_memory_merge_thread(pints);
  let (files, h4) = start_index_writer_thread(gallons, &output_dir);
  let result = merge_index_files(files, &output_dir);

  // 스레드가 끝나길 기다린다. 그 사이 발생한 오류는 담아 둔다.
  let r1 = h1.join().unwrap();
  h2.join().unwrap();
  h3.join().unwrap();
  let r4 = h4.join().unwrap();

  // 오류가 발생했다면, 그 오류를 반환한다.
  // h2, h3은 실패할 수 없다. 메모리 내 데이터만 처리한다.
  r1?;
  r4?;
  result
}

fn main() {
    let filenames = vec!["foo.txt", "bar.txt", "baz.txt"];
    process_files(filenames);
}
