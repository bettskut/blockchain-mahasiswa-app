# 🎓 Soroban Mahasiswa App

A Rust-based smart contract built using **Soroban (Stellar)** for managing student data on-chain.
This project implements a simple **CRUD (Create, Read, Delete)** system using blockchain storage.

---

## 🚀 Features

* ✅ Add new student data
* ✅ View all students
* ✅ Delete student by ID
* ✅ Auto-increment ID system
* ✅ On-chain storage using Soroban

---

## 🧱 Tech Stack

* **Language**: Rust (`#![no_std]`)
* **Framework**: Soroban SDK
* **Blockchain**: Stellar (Soroban Smart Contract)

---

## 📦 Data Structure

```rust
pub struct Mahasiswa {
    id: u64,
    nama: String,
    nim: String,
    jurusan: String,
}
```

---

## ⚙️ Smart Contract Functions

### 1. Get All Students

```rust
get_mahasiswa(env: Env) -> Vec<Mahasiswa>
```

Mengambil semua data mahasiswa dari storage.

---

### 2. Add Student

```rust
tambah_mahasiswa(env: Env, nama: String, nim: String, jurusan: String) -> String
```

Menambahkan data mahasiswa baru dengan ID otomatis.

---

### 3. Delete Student

```rust
hapus_mahasiswa(env: Env, id: u64) -> String
```

Menghapus mahasiswa berdasarkan ID.

---

## 🛠️ Installation & Setup

### 1. Install Rust

```bash
curl https://sh.rustup.rs -sSf | sh
```

---

### 2. Install Soroban CLI

```bash
cargo install soroban-cli
```

---

### 3. Build Project

```bash
cargo build --target wasm32-unknown-unknown --release
```

---

### 4. Run Tests

```bash
cargo test
```

---

## 🧪 Example Usage

### ➕ Add Student

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --fn tambah_mahasiswa \
  --arg "Elberth" \
  --arg "123456" \
  --arg "Teknologi Informasi"
```

---

### 📄 Get All Students

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --fn get_mahasiswa
```

---

### ❌ Delete Student

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --fn hapus_mahasiswa \
  --arg 1
```

---

## 📁 Project Structure

```
.
├── src/
│   ├── lib.rs        # Smart contract utama
│   └── test.rs       # Unit testing
├── Cargo.toml
└── README.md
```

---

## 🔒 Notes

* ID mahasiswa menggunakan sistem auto-increment (bukan random)
* Data disimpan langsung di blockchain (on-chain storage)
* Tidak menggunakan `std` karena berjalan di environment WASM

---

## 🚧 Future Improvements

* ✏️ Update/Edit data mahasiswa
* 🔍 Searching (Sequential & Binary Search)
* 📊 Sorting (Insertion Sort, Selection Sort)
* 🔐 Authentication (admin/user)

---

## 👨‍💻 Author

**Elberth Natan Pratama Limbong**
Teknologi Informasi - Telkom University

---

## ⭐ Support

If you find this project helpful, feel free to give it a ⭐ on GitHub!
