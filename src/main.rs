use std::fs::File;
use std::io::{BufRead, BufReader};

fn reader_sync() -> std::io::Result<()>{
    use std::time::Instant;
    let now = Instant::now(); //this will measure the time for the function to run
    println!("Sequential:");

    let files = vec!["yellowwallpaper.txt", "prayer.txt", "prophet.txt"];
    let mut filenumber = 0;

    for file in files {
        let file = File::open(file)?; //will return error if file is not found
        let reader = BufReader::new(file);

        let mut count = 0;
        for line in reader.lines() { //reads each file sequentially from the vector
            let line = line.expect("Failed to read line");
            let words = line.split_whitespace();
            for word in words {
                if word == "the" {
                    count += 1;
                }
                else if word == "The" {
                    count += 1;
                }
            }
        }
        println!("The word 'the' occurs {} times in file {}.", count, filenumber);
        filenumber += 1;
    }

    let elapsed = now.elapsed();
    println!("Elapsed time: {:.2?} \n", elapsed); //prints the time it took to run the entire function
    Ok(())
}

fn reader_task_parallelism() -> std::io::Result<()>{
    use rayon::prelude::*;
    use std::thread;
    use std::time::Instant;

    let now = Instant::now(); //this will measure the time for the function to run
    println!("Task Parallelism:");

    let files = vec!["yellowwallpaper.txt", "prayer.txt", "prophet.txt"];
    let mut filenumber = 0;

    let filess: Vec<(usize, u32)> = files.par_iter().enumerate().map(|(i, filename)| {
        let file = File::open(filename).expect("Failed to open file"); //will return error if file is not found
        let reader = BufReader::new(file);
        let mut count = 0;

        for line in reader.lines() {
            let line = line.expect("Failed to read line");
            let words = line.split_whitespace();
            for word in words {
                if word == "the" {
                    count += 1;
                }
                else if word == "The" {
                    count += 1;
                }
            }
        }
        (i, count)
    }).collect();

    for (i, count) in filess {
        println!("The word 'the' occurs {} times in file {}.", count, filenumber);
        filenumber += 1;
    }

    let elapsed = now.elapsed();
    println!("Elapsed time: {:.2?} \n", elapsed); //prints the time it took to run the entire function
    Ok(())
}

fn reader_pipeline() {
    use std::sync::mpsc::{channel, Sender, Receiver};
    use std::thread;
    use std::time::Instant;

    let now = Instant::now();
    println!("Pipeline Parallelism:");

    enum Message {
        FileLine(String),
        CountFilenumber(usize, usize),
        Exit,
    }

    struct line_reader {
        sender: Sender<Message>, //sends the message to the receiver
    }

    impl line_reader { //this function reads the lines from each file. It will send the count to the word counter where the words will be counted.
        fn run(&self, files: &[&str]) {
            let mut filenumber = 0;
            for file in files {
                let file = File::open(file).expect("Failed to open file"); //will return error if file is not found
                let reader = BufReader::new(file);
                let mut count = 0;

                for line in reader.lines() {
                    let line = line.expect("Failed to read line");
                    let words = line.split_whitespace();
                    for word in words {
                        if word == "the" {
                            count += 1;
                        }
                        else if word == "The" {
                            count += 1;
                        }
                    }
                }
                self.sender.send(Message::CountFilenumber(count, filenumber)).unwrap();
                filenumber += 1;
            }
            self.sender.send(Message::Exit).unwrap();
        }
    }

    struct word_printer {
        receiver: Receiver<Message>,
    }

    impl word_printer {
        fn run(&self) {
            let mut count = 0;
            let mut filenumber = 0;
            loop {
                match self.receiver.recv() {
                    Ok(message) => {
                        match message {
                            Message::CountFilenumber(c, f) => {
                                count = c;
                                filenumber = f;
                            }
                            Message::Exit => {
                                break;
                            }
                            _ => {}
                        }
                        println!("The word 'the' occurs {} times in file {}.", count, filenumber);
                    }
                    Err(_) => {
                        break;
                    }
                }
            }
        }
    }

    let files = vec!["yellowwallpaper.txt", "prayer.txt", "prophet.txt"];

    let (sender, receiver) = channel();
    let reader = line_reader { sender };
    let printer = word_printer { receiver };

    let files_clone = files.clone();
    let reader_thread = thread::spawn(move || reader.run(&files_clone));
    let printer_thread = thread::spawn(move || printer.run());

    reader_thread.join().unwrap();
    printer_thread.join().unwrap();

    let elapsed = now.elapsed();
    println!("Elapsed time: {:.2?} \n", elapsed); //prints the time it took to run the entire function
}

fn main() {
    reader_sync().expect("Failed to read a file.");
    reader_task_parallelism().expect("Failed to read a file.");
    reader_pipeline();
}