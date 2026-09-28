<!-- PROJECT SHIELDS -->
[![project_license][license-shield]][license-url]



<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/aviad1b/pngfier">
    <img src="images/logo.png" alt="Logo" width="80" height="80">
  </a>

<h3 align="center">PNGfier</h3>

  <p align="center">
    Compress files into a single PNG file.
    <br />
    <a href="https://github.com/aviad1b/pngfier"><strong>Explore the docs »</strong></a>
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#requirements">Requirements</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li>
		<a href="#usage">Usage</a>
		<ul>
			<li><a href="#cli">CLI</a></li>
		</ul>
	</li>
	<li>
		<a href="#technical-details">Technical details</a>
		<ul>
			<li>
        <a href="#chunk-mapping">Chunk Mapping</a>
        <ul>
          <li><a href="#overview">Overview</a></li>
          <li><a href="#pipeline">Pipeline</a></li>
          <li><a href="#stage-1-reach-computation">Stage 1: Reach computation</a></li>
          <li><a href="#stage-2-chunk-stitching">Stage 2: Chunk stitching</a></li>
          <li><a href="#complexity">Complexity</a></li>
        </ul>
      </li>
		</ul>
	</li>
  <li><a href="#license">License</a></li>
  <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

[![Project Name][project-name]]([https://example.com](https://github.com/aviad1b/pngfier))

This project allows to compress files into a single PNG file.

The input file(s) is divided into chunks that exist in a source image.
A reference map of such chunks is then stored, either to a separate file, or (preferably) onto the output PNG using PNG riding.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



### Built With

* Rust
  * Anyhow
  * Clap

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- GETTING STARTED -->
## Getting Started

Below are instructions on how to clone, compile, and run the project.

### Requirements

* [Rust](https://www.rust-lang.org/tools/install) with support for the **Rust 2024 edition**.
* Cargo (included with the Rust toolchain).

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/aviad1b/pngfier.git
   cd pngfier
   ```

2. Build the project:

   ```bash
   cargo build
   ```

   This builds both the `pngfier-core` library (containing core logic) and the `cli` executable 
   (containing a basic command-line interface).

3. Run the CLI:

   ```bash
   cargo run -p cli -- --help
   ```

   This displays the available commands and options.

### Building for Release

To compile an optimized release build:

```bash
cargo build --release
```

The resulting executable can be found under:

```text
target/release/
```

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- USAGE INSTRUCTIONS -->
## Usage

### CLI

- **NOTE**: For simplicity, these instructions assume the CLI executable is available under the name/alias `pngfier`.

#### Getting Help

The following command can be used for getting general help (seeing existing commands, etc.):

```
pngfier --help
```

To get help with a specific command, use the command:

```
pngfier <command> --help
```

E.g., to get help with the `compile` command:

```
pngfier compile --help
```

#### Compile

This command compiles a given file into a PNG file containing equivalent data.

```
pngfier compile <out-img> <in-file> {--img-query <img-query> | --img-path <img-path>} [--key-file <key-file>]
```

Arguments:

- `out-img` - An output path to save result PNG to.
- `in-file` - An input file to compile into a PNG.
- `img-query` or `img-path` - Use `img-path` to base the result PNG on an input PNG image.
  Meaning, the resulting PNG will look the same as the provided source.
  Use `img-query` to base the result PNG on a textual query (theme) - for now, this feature is not yet supported.
- `key-file` - If provided, the result's chunk map will be stored to the provided key file path (instead of using PNG riding).
  Compiling without a key file is not yet supported.

For instance, to compile `input.txt` into `output.png` based on source image `source.png` and using key file `key`, use the command:
```
pngfier compile ./output.png ./input.txt --img-path ./source.png --key-file ./key
```

The command above will generate a file `output.png` with an image that looks like `source.png`, 
from which the original `input.txt` can be extracted using `pngfier extract` (see below) and the key file `key`.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



#### Extract

This extract a file's data back from a compiled PNG.

```
pngfier extract <in-img> <out-file> [<key-file>]
```

Arguments:

- `in-img` - Path to PNG generated by `pngfier compile`.
- `out-file` - An output path to save extracted data to.
- `key-file` - If provided, the result's chunk map will be read from the provided key file path (instead of assuming PNG riding).
  Extracting without a key file is not yet supported.

For instance, to extract `input.png` into `output.txt` using key file `key`, use the command:
```
pngfier extract ./input.png ./output.txt --key-file ./key
```

The command above will generate a file `output.txt` with data extracted from `input.png`
(assuming `input.png` was originally generated using `pngfier compile`), using the key file `key`.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- TECHNICAL DETAILS -->
## Technical Details

### Chunk Mapping

#### Overview

This project represents one file (`data`) as a sequence of references into another file (`image`), 
plus literal fallbacks for any bytes that can't be referenced. 
The goal is to minimize the number of chunks needed to fully reconstruct `data`, 
while keeping runtime practical on real-sized files.

Conceptually, this is the same problem as dictionary-based delta compression (as in `xdelta`/VCDIFF or `bsdiff`): 
`image` acts as a dictionary, and `data` is encoded as a series of "copy from the dictionary" and "insert these literal elements" instructions.
Each such "instruction" is referred to as a "chunk".

A chunk is one of two kinds:

```rust
pub enum ChunkInfo<E> {
    Reference { index: ChunkIndex, size: ChunkSize }, // copy `size` elements from image[index..index+size]
    Literal(Vec<E>),                                  // write these exact, literal elements
}
```

Concatenating the chunks in order exactly reconstructs `data`.

#### Pipeline

The mapping happens in two independent stages:

1. **Reach computation** (`ReachMapper`): For every position `i` in `data`, determine the longest run starting at `i` that also 
   occurs somewhere in `image`, and where in `image` it occurs.
2. **Chunk stitching** (`ChunkMapper`): Given that reach information, greedily assemble the fewest possible chunks that together 
   cover all of `data`.

Splitting the problem this way keeps the two concerns independent - a different `ReachMapper` implementation 
(a different matching strategy, e.g. suffix-array- or hash-based) can be swapped in without touching the stitching logic at all.

#### Stage 1: Reach computation

##### The `ReachMapper` trait

```rust
pub struct MatchInfo {
    pub reach: ChunkIndex,     // furthest position in `data` this match covers
    pub src_start: ChunkIndex, // where in `image` this match starts
}

pub trait ReachMapper<E: Elem> {
    fn len(&self) -> ChunkIndex;
    fn get(&self, index: ChunkIndex) -> MatchInfo;
    fn get_elems(&mut self, start: ChunkIndex, count: ChunkSize) -> Result<Vec<E>, io::Error>;
}
```

- `get(i)` answers: "starting exactly at `data` position `i`, what's the best available match, and how far does it reach?" 
  If nothing matches at `i` at all, it returns `MatchInfo { reach: i, src_start: -1 }` - no progress.

- `get_elems` exists for the literal fallback: when no match exists, the chunk stitcher needs the actual element values at 
  that position, not just match metadata.

##### `MatrixBasedReachMapper`

The current implementation indexes `image` by **adjacent element pairs (bigrams)**. 
For every pair `(prev, curr)` that occurs in `image`, it records every position where that pair occurs:

```
img_matrix[prev][curr] = { positions in image where curr immediately follows prev }
```

For a given `data` position, this gives a set of *candidate* starting positions in `image` - every place the same first two elements occur.
From there:

1. **Prepare**: Look up `img_matrix[data[i]][data[i+1]]` to get all candidate `src_start` positions.
2. **Walk**: Advance through `data` one element at a time, checking whether each successive bigram is still consistent with 
   each candidate's continuation in `image`. Candidates that stop matching are dropped; the walk continues with whatever's left.
3. **Find best**: Because a candidate can be the longest-so-far and then later fail (diverge from `image` at some later position), 
   the *longest length seen at any point* is tracked continuously, not just among final survivors.

This trades memory for match quality: `img_matrix` is only `|alphabet|²` sets, independent of `image`'s length, which matters when `image` is large.
The cost is that the initial seed is only 2 elements long, so short accidental matches can produce more candidates to walk than a longer seed would.
A genuinely absent bigram (both elements exist individually in `image`, but never adjacent) still correctly reports no match, 
since the lookup is keyed on the pair, not either element alone.

##### Literal fallback at this stage

It cannot be guarenteed that everything in `data` has a match in `image`:

- An element's value might not appear anywhere in `image` at all.
- A specific *pair* of elements might never be adjacent in `image`, even if each element individually is present.
- The very last element of `data` has no successor to pair with, so it can never seed a match, regardless of its value.

In all of these cases, `get(i)` reports `reach: i` (no progress), and the chunk stitcher (below) falls back to a literal.

#### Stage 2: Chunk stitching

##### The core algorithm

Given `reach[i]` for every position, finding the *minimum* number of chunks to cover `data` end-to-end is the same problem as 
**Jump Game II** / minimum interval cover: you're allowed to "jump" from position `i` to any point up to `reach[i]`, 
and you want to cover the whole range in as few jumps as possible.

The greedy solution is a single forward pass: track the furthest-reaching match seen so far without ever resetting it, 
and only "commit" a chunk once the scan catches up to the current frontier (`cur_end`):

```
i = 0, cur_end = 0, farthest = no match yet

while i < len(data):
    if reach(i) beats farthest: farthest = reach(i)
    if i == cur_end:
        commit a chunk using `farthest`
        cur_end = farthest.reach
    i += 1
```

This greedy strategy is provably optimal: by the time a chunk is committed at `cur_end`, 
every candidate starting anywhere from `0` up to `cur_end` has already been examined, 
so `farthest` is guaranteed to be the best reachable option - not just the best one starting exactly at `cur_end`.

##### Trimming overlapping matches

A subtlety: the winning match might have been *found* at a position earlier than `cur_end` 
(i.e. it started inside territory the previous chunk already covered). 
Emitting the whole match would duplicate already-output data. 
The fix is to trim the match down to only its unclaimed suffix:

```
offset = cur_end - farthest_data_start
chunk = Reference { index: farthest.src_start + offset, size: farthest.reach - cur_end }
```

Since a verified matching run's suffix is itself still a valid match 
(`image[src+t] == data[data_start+t]` for `t` in range implies the same holds for any sub-range), 
this trimming is always safe.

##### Chunk size bounds

Two optional (as far as the `core` crate is concerned) parameters shape the final chunk list:

- **`min_cap`**: A reference chunk shorter than this is rejected in favor of a literal. 
  Below some size, the overhead of storing an `{index, size}` pair isn't worth it compared to just storing the element(s) directly. 
  The comparison is strict (`min_cap < size`), so a chunk exactly equal to `min_cap` is rejected, not kept.
- **`max_cap`**: A reference chunk longer than this is split into multiple `Reference` entries, each at most `max_cap` in size. 
  This is useful when the encoding or transport format has a maximum representable chunk size. 
  The final piece of a split run isn't re-checked against `min_cap` - the *whole* run already cleared that bar before splitting, 
  and re-litigating the threshold per-fragment would cause a single long match to partially degrade into literals for no benefit.

Both are `Option<ChunkSize>`; passing `None` disables the corresponding bound.

The former parameter is used to prevent mapping a "reference" chunk in a scenario where a "literal" chunk would take less space; 
the latter is used to prevent integer overflow (since the size stored in memory has a maximum limit).

#### Complexity

For an `image` of length `n` and `data` of length `m`:

- Building `img_matrix`: O(n).
- Reach computation: O(m) seed lookups, each followed by a walk whose cost depends on how many candidates a given bigram has 
  and how far they extend before diverging. Worst case (highly repetitive `image`, e.g. long runs of a single byte) 
  degrades toward O(n·m); this is a known trade-off of bigram-based seeding versus a longer/more selective seed length.
- Chunk stitching: O(m), single forward pass, no backtracking.


<!-- ROADMAP -->
## Roadmap

- [X] Version 1.0.0
  - [X] Basic CLI
  - [X] Chunk Mapping
- [ ] Version 1.1.0
  - [ ] PNG-riding-based key storage &amp; retreival
  - [ ] Query-based image selection

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- LICENSE -->
## License

This project is protected under the GPL-3.0 License. See `LICENSE` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

* [Best-README-Template by @othneildrew](https://github.com/othneildrew/Best-README-Template?tab=readme-ov-file)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[license-shield]: https://img.shields.io/github/license/aviad1b/pngfier.svg?style=for-the-badge
[license-url]: https://github.com/aviad1b/pngfier/blob/master/LICENSE
[project-name]: images/proj_name.png
