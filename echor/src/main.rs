use clap::{App, Arg};

fn main() {

    let matches = App::new("echor")
        .version("0.1.0")
        .author("Bling-Bling-Dong <whoareyouman0.4@gmail.com")
        .about("以Rust驅動的echo")
        .arg (
          Arg::with_name("text")
              .value_name("Text")
              .help("輸入文字")
              .required(true)
              .min_values(1),
        )
        .arg(
          Arg::with_name("omit_newline")
              .short("n")
              .help("不換行")
              .takes_value(false),
         )
        .get_matches();
        
     let text = matches.values_of_lossy("text").unwrap();
     let omit = matches.is_present("omit_newline");
     
      //let format_text = text.iter().fold(String::new(), |s, t| s + t +" ");
     let format_text = text.join(" ");
     
     /*
     if omit {
       print!("{}\n", format_text);
     } else{
       print!("{}", format_text);
     }
     */
     let ending = if omit {""} else {"\n"};
     print!("{}{}", format_text, ending);
    
     
}



