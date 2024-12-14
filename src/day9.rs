#[cfg(test)]
mod test {
    use crate::util::read_file_to_string_array;
    use std::collections::VecDeque;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct SpaceInfo {
        position: i64,
        length: i64,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct FileInfo {
        id: i64,
        position: i64,
        length: i64,
    }

    fn get_files_and_spaces(blocks: &Vec<u8>) -> (Vec<FileInfo>, Vec<SpaceInfo>) {
        let mut files: Vec<FileInfo> = Vec::new();
        let mut spaces: Vec<SpaceInfo> = Vec::new();
        let mut fileid = 0;
        let mut position = 0;
        let mut is_file = true;
        for block in blocks {
            if is_file {
                files.push(FileInfo { id: fileid, position: position, length: *block as i64 });
                position += *block as i64;
                fileid += 1;
            } else {
                spaces.push(SpaceInfo { position: position, length: *block as i64 });
                position += *block as i64;
            }
            is_file = !is_file;
        }
        (files, spaces)
    }

    fn accumulate_files_byblocks(files: &Vec<FileInfo>, spaces: &Vec<SpaceInfo>) -> i64 {
        let mut work_files: VecDeque<FileInfo> = VecDeque::from_iter(files.iter().cloned());
        let mut work_spaces: Vec<SpaceInfo> = spaces.clone();

        let mut accum:i64 = 0;
        let mut position = 0;
        let mut remainging_file:Option<FileInfo> = None;
        loop {
            // Work on accumulating file
            let cur_file:FileInfo = work_files.pop_front().unwrap();
            for idx in position..position+cur_file.length{
                accum += cur_file.id * idx;
            }
            position += cur_file.length;

            // Work on accumulating last file over this space
            let space = work_spaces.remove(0);
            let mut space_remaining = space.length;
            loop {
                let fill_file = match remainging_file {
                    Some(file) => file,
                    None => if work_files.len() == 0 {
                        break;
                    } else {
                        work_files.pop_back().unwrap()
                    }
                };
                let fill_length = std::cmp::min(space_remaining, fill_file.length);
                for idx in position..position+fill_length{
                    accum += fill_file.id * idx;
                }
                position += fill_length;
                space_remaining -= fill_length;
                if fill_file.length > fill_length {
                    remainging_file = Some(FileInfo { id: fill_file.id, position: position, length: fill_file.length - fill_length });
                } else {
                    remainging_file = None;
                }
                if space_remaining == 0 {
                    break;
                }
            }

            if work_files.len() == 0 {
                break;
            }
        }

        // Now count any remaining blocks
        match remainging_file {
            Some(file) => {
                for idx in position..position+file.length{
                    accum += file.id * idx;
                }
            },
            None => ()
        }

        accum
    }

    fn accumulate_files_bestfit(files: &Vec<FileInfo>, spaces: &Vec<SpaceInfo>) -> i64 {
        let mut work_files = files.clone();
        let mut work_spaces = spaces.clone();
        for idx in (1..work_files.len()).rev() {
            let file = work_files[idx];
            let mut sdx = 0;
            while sdx < work_spaces.len() && work_spaces[sdx].position < file.position {
                let space = work_spaces[sdx];
                if file.length <= space.length {
                    work_files[idx].position = space.position;
                    work_spaces[sdx].position += file.length;
                    work_spaces[sdx].length -= file.length;
                    break;
                }
                sdx += 1;
            }
        }

        let mut accum: i64 = 0;
        work_files.sort_by(|a,b| a.position.cmp(&b.position));
        for file in work_files {
            // println!("File: {:?}", file);
            for idx in file.position..file.position+file.length {
                accum += file.id * idx;
            }
        }

        accum
    }

    #[test]
    fn day9_part1(){
        // let lines = read_file_to_string_array("src/day9_test.data").unwrap();
        let lines = read_file_to_string_array("src/day9_part1.data").unwrap();
        let blocks: Vec<u8> = lines[0].chars().map(|x| x.to_digit(10).unwrap() as u8).collect();
        let (files, spaces) = get_files_and_spaces(&blocks);
        let accum = accumulate_files_byblocks(&files, &spaces);
        println!("The accum is: {}", accum);
    }

    #[test]
    fn day9_part2(){
        // let lines = read_file_to_string_array("src/day9_test.data").unwrap();
        let lines = read_file_to_string_array("src/day9_part1.data").unwrap();
        let blocks: Vec<u8> = lines[0].chars().map(|x| x.to_digit(10).unwrap() as u8).collect();
        let (files, spaces) = get_files_and_spaces(&blocks);
        let accum = accumulate_files_bestfit(&files, &spaces);
        println!("The accum is: {}", accum);
    }
}