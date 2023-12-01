alias r := run-day
alias rr := run-release-day
alias b := build-day
alias br := build-release-day
alias d := download-day
alias s := submit-day
alias t := test-day
alias tr := test-release-day

add-day day:
	cp src/temp.rs src/day{{day}}.rs
	echo '[[bin]]\nname="day{{day}}"\npath="src/day{{day}}.rs"\n' >> Cargo.toml

build-day day:
	@ cargo build --bin day{{day}}

run-day day: (build-day day)
	@ ./target/debug/day{{day}}

build-release-day day:
	@ cargo build --release --bin day{{day}}

run-release-day day: (build-release-day day)
	@ ./target/release/day{{day}}

download-day day year='2023':
	aoc d -d {{day}} -y {{year}} -f input/day{{day}}.txt

submit-day day part res year='2023':
	aoc s -d {{day}} -y {{year}} {{part}} {{res}}

test-day day: (build-day day)
	@ cat ./input/day{{day}}.txt | ./target/debug/day{{day}}

test-release-day day: (build-release-day day)
	@ cat ./input/day{{day}}.txt | ./target/release/day{{day}}
