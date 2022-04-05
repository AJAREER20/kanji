use structopt::StructOpt;

mod daily;
mod printcurr;
mod finish;
mod search;
#[derive(Debug, StructOpt)]
#[structopt(name = "Kanji", about = "A dictionary for kanji")]
enum Opt {
	
    daily,
	current,
	finish,
	search{
			kanji:char
	},
}

fn main() {
    let opt = Opt::from_args();

	match opt{
			Opt::daily => daily::daily(),
			Opt::current => printcurr::printc(),
			Opt::finish => finish::finish(),
			Opt::search {kanji} => search::search(kanji)
	}
}

