#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data Mahasiswa
#[contracttype]
#[derive(Clone, Debug)]
pub struct Mahasiswa {
    id: u64,
    nama: String,
    nim: String,
    jurusan: String,
}

// Storage key
const MHS_DATA: Symbol = symbol_short!("MHS_DATA");
const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct MahasiswaContract;

#[contractimpl]
impl MahasiswaContract {

    // Ambil semua data mahasiswa
    pub fn get_mahasiswa(env: Env) -> Vec<Mahasiswa> {
        env.storage()
            .instance()
            .get(&MHS_DATA)
            .unwrap_or(Vec::new(&env))
    }

    // Tambah mahasiswa
    pub fn tambah_mahasiswa(env: Env, nama: String, nim: String, jurusan: String) -> String {
        let mut data: Vec<Mahasiswa> = env
            .storage()
            .instance()
            .get(&MHS_DATA)
            .unwrap_or(Vec::new(&env));

        // ambil counter
        let mut counter: u64 = env
            .storage()
            .instance()
            .get(&COUNTER)
            .unwrap_or(0);

        counter += 1;

        let mhs = Mahasiswa {
            id: counter,
            nama,
            nim,
            jurusan,
        };

        data.push_back(mhs);

        // simpan kembali
        env.storage().instance().set(&MHS_DATA, &data);
        env.storage().instance().set(&COUNTER, &counter);

        String::from_str(&env, "Mahasiswa berhasil ditambahkan")
    }

    // Hapus mahasiswa berdasarkan id
    pub fn hapus_mahasiswa(env: Env, id: u64) -> String {
        let mut data: Vec<Mahasiswa> = env
            .storage()
            .instance()
            .get(&MHS_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..data.len() {
            let mhs = data.get(i).unwrap();

            if mhs.id == id {
                data.remove(i);

                env.storage().instance().set(&MHS_DATA, &data);

                return String::from_str(&env, "Mahasiswa berhasil dihapus");
            }
        }

        String::from_str(&env, "Mahasiswa tidak ditemukan")
    }
}

mod test;