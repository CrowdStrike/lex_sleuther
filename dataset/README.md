# lex_sleuther dataset

In order to add new classification categories or add new samples to the training set, you'll need to retrain. To build `lex_sleuther` with training features, you'll need to select an external LAPACK implementation for [`ndarry-linalg`](https://github.com/rust-ndarray/ndarray-linalg?tab=readme-ov-file#backend-features) to use.

A LAPACK backend was not selected by default due to potential platform compatibility and licensing issues. The best option on Linux platforms is `openblas-static`, but this does require `gcc`, `gfortran`, and `make`. 

```bash
cargo install -F train,ndarray-linalg/openblas-static --path .
```

You may select to download and use `intel-mkl-static` backend for broader platform support (MacOS, Windows, etc), but ensure that you have read and agree to the [Intel Simplified Software License](https://www.intel.com/content/www/us/en/developer/articles/license/end-user-license-agreement.html) first. 


## preparing

To prepare a corpus of sample files, you'll want to organize your training files into folders based on their class. To train effectively, at least 1000 sample files are required at an absolute minimum, but >5000 is recommended. 

Here's an example layout.

```rust
/dataset
    /samples
        /bat
            file1.bat
            ...
        /html
            file2.html
            ...
        /python
            file3.py
            ...
        ...
```

Next, train the model like so.

```bash
lex_sleuther train -v --output-path ./src/model/baked.rs ./dataset/samples/*
```

Currently, this will emit Rust source code and replace the existing model. The ability to import different models at runtime is a TODO item.

### do I need to create a new lexer for a new language?

Technically, there is no requirement that you train on a set of folders that corresponds 1-1 with the lexers supported by this project. It is possible for instance to add a folder containing `css` files and include it in the training set; the resulting model will conclude `css` where appropriate. That said, you can expect to see a small decline in overall accuracy if you train on an unsupported language.