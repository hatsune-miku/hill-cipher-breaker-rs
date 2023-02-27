use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;

trait Mod {
    fn mod_by(&self, x: i32) -> Self;
}

impl Mod for i32 {
    fn mod_by(&self, x: i32) -> Self {
        ((self % x) + x) % x
    }
}

trait Matrix {
    fn mul<T: Matrix>(&mut self, other: &T);
    fn mod_by(&mut self, modulo: i32);
    fn at(&self, row: usize, col: usize) -> i32;
    fn set(&mut self, row: usize, col: usize, value: i32);
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn det(&self) -> i32;
    fn det_inverse(&self) -> i32;
    fn inversed(&self) -> Self;

    fn square(&self) -> bool {
        self.width() == self.height()
    }
}

impl Matrix for Vec<i32> {
    fn mul<T: Matrix>(&mut self, o: &T) {
        if self.width() != o.height() {
            panic!("Can not multiply matrices.");
        }
        let mut result = vec![0; o.width()];
        for i in 0..o.width() {
            for j in 0..self.width() {
                result[i] += self[j] * o.at(j, i);
            }
        }
        *self = result;
    }

    fn mod_by(&mut self, modulo: i32) {
        for i in 0..self.len() {
            self[i] = self[i].mod_by(modulo);
        }
    }

    fn at(&self, row: usize, col: usize) -> i32 {
        if row != 0 {
            panic!("Invalid row index.");
        }
        self[col]
    }

    fn set(&mut self, row: usize, col: usize, value: i32) {
        if row != 0 {
            panic!("Invalid row index.");
        }
        self[col] = value;
    }

    fn width(&self) -> usize {
        self.len()
    }

    fn height(&self) -> usize {
        1
    }

    fn det(&self) -> i32 {
        panic!("Can not calculate determinant of a vector.");
    }

    fn det_inverse(&self) -> i32 {
        panic!("Can not calculate determinant inverse of a vector.");
    }

    fn inversed(&self) -> Self {
        panic!("Can not calculate inverse of a vector.");
    }
}

struct HillCipherKeyMatrix {
    modulo: i32,
    a: i32,
    b: i32,
    c: i32,
    d: i32,
}

impl Clone for HillCipherKeyMatrix {
    fn clone(&self) -> Self {
        Self { modulo: self.modulo, a: self.a, b: self.b, c: self.c, d: self.d }
    }
}

impl Debug for HillCipherKeyMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}; {}, {}]", self.a, self.b, self.c, self.d)
    }
}

impl Matrix for HillCipherKeyMatrix {
    fn mul<T: Matrix>(&mut self, _: &T) {
        panic!("I am key.")
    }

    fn mod_by(&mut self, modulo: i32) {
        self.a = self.a.mod_by(modulo);
        self.b = self.b.mod_by(modulo);
        self.c = self.c.mod_by(modulo);
        self.d = self.d.mod_by(modulo);
    }

    fn at(&self, row: usize, col: usize) -> i32 {
        match (row, col) {
            (0, 0) => self.a,
            (0, 1) => self.b,
            (1, 0) => self.c,
            (1, 1) => self.d,
            _ => panic!("Invalid index."),
        }
    }

    fn set(&mut self, row: usize, col: usize, value: i32) {
        match (row, col) {
            (0, 0) => self.a = value,
            (0, 1) => self.b = value,
            (1, 0) => self.c = value,
            (1, 1) => self.d = value,
            _ => panic!("Invalid index."),
        }
    }

    fn width(&self) -> usize {
        2
    }

    fn height(&self) -> usize {
        2
    }

    fn det(&self) -> i32 {
        (self.a * self.d - self.b * self.c).mod_by(self.modulo)
    }

    fn det_inverse(&self) -> i32 {
        let det = self.det();
        (0..self.modulo).find(|&inv| (det * inv).mod_by(self.modulo) == 1)
            .expect("Error in det_inverse")
    }

    fn inversed(&self) -> Self {
        let det_inv = self.det_inverse();
        Self::from(
            (self.d * det_inv).mod_by(self.modulo),
            (-self.b * det_inv).mod_by(self.modulo),
            (-self.c * det_inv).mod_by(self.modulo),
            (self.a * det_inv).mod_by(self.modulo),
            self.modulo,
        )
    }
}

impl HillCipherKeyMatrix {
    fn from(a: i32, b: i32, c: i32, d: i32, modulo: i32) -> Self {
        Self { modulo, a, b, c, d }
    }

    fn update(&mut self, a: i32, b: i32, c: i32, d: i32) {
        self.a = a;
        self.b = b;
        self.c = c;
        self.d = d;
    }
}

struct B2 {
    pub ciphertext: String,
}

impl B2 {
    fn from(filename: &str) -> Self {
        let ciphertext = fs::read_to_string(filename)
            .expect("Error in reading file")
            .replace("\n", "")
            .to_uppercase();
        Self { ciphertext }
    }

    fn top_digrams(&self) -> Vec<(String, i32)> {
        let mut freq = HashMap::<String, i32>::new();

        for i in (0..self.ciphertext.len()).step_by(2) {
            let block = self.ciphertext[i..i+2].to_string();
            let current = freq.get(&block).unwrap_or(&0);
            freq.insert(block, current + 1);
        }

        let mut ret = freq
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect::<Vec<(String, i32)>>();
        ret.sort_by(|(_, v1), (_, v2)| v2.cmp(v1));
        ret
    }

    fn solve(
        &self,
        plaintext_blocks: &[Vec<i32>],
        ciphertext_blocks: &[Vec<i32>],
        modulo: i32
    ) -> Option<HillCipherKeyMatrix> {
        let mut key_enc = HillCipherKeyMatrix::from(0, 0, 0, 0, modulo);
        for i in 0..modulo {
            for j in 0..modulo {
                for k in 0..modulo {
                    for l in 0..modulo {
                        key_enc.update(i, j, k, l);
                        if plaintext_blocks.iter().all(|pb| {
                            ciphertext_blocks.contains(
                                &crypt(pb, &key_enc, modulo))
                        }) {
                            return Some(key_enc);
                        }
                    }
                }
            }
        }
        None
    }
}

fn crypt(block: &Vec<i32>, key: &HillCipherKeyMatrix, modulo: i32) -> Vec<i32> {
    let mut ret = block.clone();
    ret.mul(key);
    ret.mod_by(modulo);
    ret
}

fn string_view(blocks: &[Vec<i32>]) -> String {
    blocks.iter().map(|block| {
        block.iter().map(|&x| (x + 65) as u8 as char)
            .collect::<String>()
    }).collect::<String>()
}

fn block_view(string: String) -> Box<[Vec<i32>]> {
    string.as_bytes()
        .chunks(2)
        .map(|block| {
            block.iter().map(|&x| (x - 65) as i32).collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
        .into_boxed_slice()
}

fn main() {
    let modulo = 26;
    let try_cipher_digrams = 10;

    println!("[-] Reading A1_ctext.txt");
    let b2 = B2::from("A1_ctext.txt");

    let cipher_digrams = b2.top_digrams();
    println!("[-] Top digrams: {:?}", cipher_digrams);

    let given_digrams = block_view("THHEINERAN".to_string());
    let cipher_digrams = cipher_digrams.iter()
        .map(|x| &x.0)
        .take(try_cipher_digrams)
        .map(|s| s.chars().map(|c| (c as i32 - 65))
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!("[-] Searching for encryption key...");
    let enc_key = match b2.solve(
        &given_digrams,
        &cipher_digrams,
        modulo
    ) {
        Some(key) => key,
        None => panic!("Key not found"),
    };
    // let enc_key = HillCipherKeyMatrix::new(vec![vec![14, 7], vec![3, 1]], modulo);

    println!("[!] Encryption key found!");
    println!("[-] Calculating corresponding decryption key...");
    let dec_key = enc_key.inversed();

    println!("[-] Encryption key: {:?}", enc_key);
    println!("[-] Decryption key: {:?}", dec_key);

    let blocks = block_view(b2.ciphertext);
    let decrypted = string_view(
        blocks.iter()
            .map(|b| crypt(b, &dec_key, modulo))
            .collect::<Vec<_>>()
            .as_slice()
    );

    fs::write("result.txt", &decrypted).unwrap();
    println!("Decrypted text written to result.txt");
}
