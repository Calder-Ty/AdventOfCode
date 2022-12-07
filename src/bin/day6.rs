const OFFSET: u8 = 97;

fn main () {
    let input_stream = include_str!("./data/day_6.prod.txt");
    for line in input_stream.lines() {
        println!("{}: {}", line, start_of_packet(line.as_bytes(), 4));
    }
    println!("*****");
    for line in input_stream.lines() {
        println!("{}: {}", line, start_of_packet(line.as_bytes(), 14));
    }
}

fn start_of_packet(packet: &[u8], window_size: usize) -> usize {
    let mut count = window_size;
    let mut index = [0; 26];
    for (i, arr) in packet.windows(window_size).enumerate() {
        for j in arr {
            index[(*j-OFFSET) as usize] += 1;
        }
        if index.iter().any(|x| *x >= 2) {
            // Reset index
            for elem in index.iter_mut() {
                *elem = 0;
            }
        } else {
            count += i;
            break
        }
    };
    count
}

