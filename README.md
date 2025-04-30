# Cricket

A programing language to compose music without complexities!

---

## 🎧 What is Cricket?

Cricket is an experimental music composition language. You write simple, readable text files that describe instruments, patterns, and songs. The compiler turns those into MIDI files.

You will be able to programatically generate your ideas and see diferent forms of it with a click. Designed to be your buddy while song writing.

It's still very early, but the goal is to make composing music feel like writing a script. If you've ever wanted to create a song by editing a file in your favorite text editor, this is for you.

## 💭 The Idea Behind It

Cricket came out of the desire to merge music with programming — especially for people who love both but aren't satisfied with the tools out there. It should be easy to:

- Write short musical ideas
- Reuse and layer them
- Version your music with Git
- Stay focused on the sound, not the UI
- Stop caring about the keys make your music flexible!
- Write basic music and get jazzified results

We believe music can be text — and that this text can be expressive, simple, and fun to use.

## 🔹 What Does a Cricket File Look Like?

A `.crkt` file defines instruments, patterns, and songs. Here’s a quick example:

```cricket
Instrument violin:
    type: Strings
    midi_path: violin.sf2

Pattern intro():
    return [1:8] Note(Am) + [1:8] Wait()

Section Verse:
    Channel melody:
        intro()

Song Raindrops:
    return Verse()
```

It’s like YAML with rhythm. Once compiled, it becomes a MIDI file you can use anywhere.

## 🚀 Where Do We Want to Go?

This is a side project, but we have ideas:

- Multiple instruments playing together
- A simple standard library of patterns
- A web editor or VS Code plugin
- Export to WAV using SoundFonts
- More expressive musical syntax

Right now, it does basic things and we’re happy with that. We want to grow slowly, with help from anyone who thinks this idea is cool.

## 🙌 How to Contribute

This is an open source project, and we'd love help. If you enjoy Rust, MIDI, or music theory, here are a few ideas:

- Add new features or syntax rules
- Write example `.cricket` files
- Help improve error messages or docs
- Suggest better file structures or grammar

No contribution is too small. Even just trying it out and opening issues helps a lot.

## 🔧 Getting Started Locally

1. **Clone the repo**:
    ```sh
    git clone https://github.com/your-username/cricket
    cd cricket
    ```

2. **Build it**:
    ```sh
    cargo build
    ```

3. **Run the compiler**:
    ```sh
    cargo run -- ./examples/intro.cricket
    ```

4. **Check the output**: A `.mid` file will be created. You can open it in a DAW or play it using a SoundFont synthesizer.

5. **Optional: Export to WAV**:
    If you have [FluidSynth](https://www.fluidsynth.org/):
    ```sh
    fluidsynth -ni assets/soundfonts/violin.sf2 output.mid -F output.wav -r 44100
    ```

---

Thanks for checking out Cricket. If this sounds like something you want to help shape, we’d love to have you.


