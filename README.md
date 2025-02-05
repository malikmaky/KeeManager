




# 🔐 KeeManager


![AGPL License](https://img.shields.io/badge/Tauri-Project-darkorange)

![Logo](https://images2.imgbox.com/a0/e8/nVoYAFnW_o.jpg)


KeeManager is a secure password and data management desktop application designed to help users securely store, manage, and access sensitive information.









## Key Features :

- #### __🔒 Secure Data Storage :__ 
AES-256 GCM encryption for strong data protection.

- #### __🔑 Master Password Security :__ 
PBKDF2_HMAC for deriving secure encryption keys.
 
- #### __🙋‍♂️ User-Friendly Interface :__

Developed with Svelte for a responsive and intuitive UI.

- #### __💻📱 Cross-Platform Compatibility :__
Built using the Tauri framework for lightweight and high-performance multi-platform support.

- #### __💡 Database Integration :__
Encrypted SQLite database for managing different types of sensitive data (passwords, notes, credit card information).

## Technical Overview :

 __Cryptography :__ 
 AES-256, PBKDF2_HMAC, SHA-256, and Base64 encoding.

__Backend :__ 
Developed in Rust for robust memory safety and performance.

__Frontend :__ 
Built with Svelte for a smooth Single Page Application (SPA) experience.

__Database :__ 
Singleton-pattern-based SQLite integration using OnceCell for efficient resource management.

## How It Works :
![Logo](https://images2.imgbox.com/c1/ad/W4UrvSj0_o.jpg)

__1. Key Derivation :__ A master password is securely processed using PBKDF2_HMAC.

__2. Data Encryption :__  Sensitive information is encrypted with AES-256 GCM and combined with a secure IV/Nonce.

__3. Data Storage :__  Encrypted data is encoded with Base64 and stored securely.

__4. Efficient Data Retrieval :__ CRUD operations allow efficient management of stored data.

## Conclusion :
Developing KeeManager with Tauri and Rust proved to be a strategic choice for building a secure, efficient, and cross-platform password management application. Rust's memory safety features and high-performance capabilities ensured a robust and secure backend, critical for handling sensitive user data. By leveraging Tauri's lightweight framework, we achieved native desktop application performance with minimal resource consumption, far outperforming traditional alternatives.

Furthermore, Tauri's seamless integration with modern web technologies like Svelte allowed us to design an intuitive and responsive user interface without sacrificing application speed. 

The experience demonstrated how Tauri and Rust can be powerful tools for projects requiring both security and an appealing user experience, striking a balance between backend reliability and frontend flexibility.

## Demo

Download the latest build from the Release section.


## Credits

This project is maintained by @malikmaky.

For any inquiries or feedback, please contact malikmhmd@hotmail.com

