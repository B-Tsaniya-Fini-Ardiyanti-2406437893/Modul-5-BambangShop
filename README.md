# BambangShop Publisher App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases and methods to access the databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a basic functionality that makes BambangShop work: ability to create, read, and delete `Product`s.
This repository already contains a functioning `Product` model, repository, service, and controllers that you can try right away.

As this is an Observer Design Pattern tutorial repository, you need to implement another feature: `Notification`.
This feature will notify creation, promotion, and deletion of a product, to external subscribers that are interested of a certain product type.
The subscribers are another Rocket instances, so the notification will be sent using HTTP POST request to each subscriber's `receive notification` address.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Publisher" folder.
This Postman collection also contains endpoints that you need to implement later on (the `Notification` feature).

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    APP_INSTANCE_ROOT_URL="http://localhost:8000"
    ```
    Here are the details of each environment variable:
    | variable              | type   | description                                                |
    |-----------------------|--------|------------------------------------------------------------|
    | APP_INSTANCE_ROOT_URL | string | URL address where this publisher instance can be accessed. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

## Mandatory Checklists (Publisher)
-   [ ] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [ ] Commit: `Create Subscriber model struct.`
    -   [ ] Commit: `Create Notification model struct.`
    -   [ ] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [ ] Commit: `Implement add function in Subscriber repository.`
    -   [ ] Commit: `Implement list_all function in Subscriber repository.`
    -   [ ] Commit: `Implement delete function in Subscriber repository.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [ ] Commit: `Create Notification service struct skeleton.`
    -   [ ] Commit: `Implement subscribe function in Notification service.`
    -   [ ] Commit: `Implement subscribe function in Notification controller.`
    -   [ ] Commit: `Implement unsubscribe function in Notification service.`
    -   [ ] Commit: `Implement unsubscribe function in Notification controller.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [ ] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [ ] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [ ] Commit: `Implement publish function in Program service and Program controller.`
    -   [ ] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1
**1. In the Observer pattern diagram explained by the Head First Design Pattern book, Subscriber is defined as an interface. Explain based on your understanding of Observer design patterns, do we still need an interface (or trait in Rust) in this BambangShop case, or a single Model struct is enough?**

**Answer:** Dalam studi kasus BambangShop, penggunaan sebuah struct model tunggal sudah cukup memadai tanpa perlu mendefinisikan interface (atau trait dalam Rust). Berdasarkan pola desain Observer, interface pada subscriber umumnya dirancang untuk mencapai polimorfisme ketika sistem memiliki berbagai jenis penerima dengan mekanisme notifikasi yang berbeda-beda. Namun, karena semua subscriber pada aplikasi BambangShop memiliki perilaku yang seragam, yaitu menerima notifikasi secara eksklusif melalui HTTP POST request ke URL Webhook, abstraksi tambahan menggunakan trait menjadi tidak diperlukan. Implementasi trait baru akan relevan dan dibutuhkan secara arsitektural apabila di masa depan sistem perlu mengakomodasi variasi tipe subscriber dengan logika pemrosesan yang berbeda, seperti pengiriman notifikasi melalui email atau SMS.

---

**2. id in Program and url in Subscriber is intended to be unique. Explain based on your understanding, is using Vec (list) sufficient or using DashMap (map/dictionary) like we currently use is necessary for this case?**

**Answer:** Penggunaan `Vec` sebenarnya cukup secara fungsional untuk menyimpan data, namun tidak efisien untuk menjamin keunikan data. Jika menggunakan `Vec`, sistem harus melakukan iterasi pencarian linear dengan kompleksitas waktu O(N) setiap kali ingin memastikan tidak ada duplikasi `id` atau `url`. Sebaliknya, penggunaan `DashMap` sangat diperlukan karena struktur data ini menggunakan sistem key-value pair yang menjamin keunikan key. Selain itu, proses pencarian dan penyisipan data pada `Map` beroperasi dengan kompleksitas waktu O(1), yang memberikan performa lebih optimal.

---

**3. When programming using Rust, we are enforced by rigorous compiler constraints to make a thread-safe program. In the case of the List of Subscribers (SUBSCRIBERS) static variable, we used the DashMap external library for thread safe HashMap. Explain based on your understanding of design patterns, do we still need DashMap or we can implement Singleton pattern instead?**

**Answer:** Singleton pattern adalah pola desain yang memastikan hanya ada satu instance objek (`SUBSCRIBERS`) yang bersifat global, sedangkan `DashMap` adalah struktur data thread-safe HashMap. Pada kenyataannya, kita tetap membutuhkan DashMap di dalam implementasi Singleton. Penggunaan `lazy_static!` sendiri sudah merupakan bentuk penerapan pola Singleton di Rust. Namun, memiliki Singleton saja tidak cukup. Web framework memproses permintaan secara asinkron dan multi-threading, sehingga jika kita menempatkan HashMap standar di dalam Singleton, kompilator Rust akan menolaknya karena tingginya risiko data race akibat mutasi dari berbagai thread. Oleh karena itu, implementasi Singleton di Rust yang menangani mutasi data konkuren mutlak membutuhkan struktur data internal yang thread-safe seperti DashMap (atau dibungkus dengan primitif sinkronisasi seperti Mutex/RwLock) agar program dapat dikompilasi dan berjalan dengan aman.

---

#### Reflection Publisher-2
**1. In the Model-View Controller (MVC) compound pattern, there is no“Service” and “Repository”. Model in MVC covers both data storage and business logic. Explain based on your understanding of design principles, why we need to separate “Service” and “Repository” from a Model?**

**Answer:** Pemisahan Service dan Repository dari Model sangat berkaitan erat dengan penerapan *Single Responsibility Principle* dan *Separation of Concerns* dalam desain perangkat lunak. Pada MVC konvensional Model melakukan penyimpanan data sekaligus *business logic*. Seiring membesarnya aplikasi, hal ini akan memicu *Fat Model* yang membuat kode sulit di maintain dan diuji secara terisolasi. Melalui pemisahan ini tanggung jawab menjadi terdistribusi dengan jelas:
1. Model hanya berfungsi sebagai representasi struktur data
2. Repository berfokus eksklusif pada akses dan manipulasi data ke database
3. Service berfokus penuh untuk menangani *business logic* dan aturan sistem.

Pemisahan ini menghasilkan arsitektur yang tingkat ketergantungan antar komponen yang rendah, di mana perubahan pada teknologi *database* hanya akan berdampak pada Repository, dan logika bisnis pada Service dapat di uji secara independen menggunakan *mock repository*.

---

**2. What happens if we only use the Model? Explain your imagination on how the interactions between each model (Program, Subscriber, Notification) affect the code complexity for each model?**

**Answer:** Jika hanya mengandalkan Model tanpa mendistribusikan tanggung jawab ke layer Service dan Repository, seluruh interaksi, manipulasi data, dan *business logic* akan tertumpuk di satu tempat. Dampak buruk dari kompleksitas kode ini, antara lain:
1. Pelanggaran Single Responsibility Principle (SRP): Model melakukan tugas-tugas yang di luar lingkupnya
2. Spaghetti Code: Ketergantungan langsung antar-model membuat program sangat sulit untuk dibaca dan dikembangkan.
3. High Testing Complexity: Sulit untuk melakukan unit testing pada model secara terisolasi, karena pengujian satu entitas akan mengharuskan kita untuk ikut melakukan mocking pada koneksi database dan respon HTTP client yang menempel padanya.

---

**3. Have you explored more about Postman? Tell us how this tool helps you to test your current work. You might want to also list which features in Postman you are interested in or feel like it is helpful to help your Group Project or any of your future software engineering projects.**

**Answer:** Pada pengerjaan saat ini, fitur Postman Collections sangat membantu untuk mengorganisasi, menyimpan, dan mengeksekusi berbagai HTTP request beserta payload JSON yang dibutuhkan secara berulang. Terdapat beberapa fitur tingkat lanjut di Postman yang sangat menarik untuk dimanfaatkan. Pertama adalah fitur Environments dan Variables, yang memungkinkan transisi pengujian API dari localhost ke server production dengan sangat mulus. Kedua adalah Automated API Testing (tab Tests), di mana kita bisa menulis script untuk memvalidasi status code dan struktur JSON secara otomatis setiap kali request dikirim. Terakhir, fitur API Documentation dan Mock Servers akan sangat memfasilitasi kolaborasi, sehingga pengembang frontend dan backend dapat menyepakati "kontrak" API secara terpusat.

---

#### Reflection Publisher-3
**1. Observer Pattern has two variations: Push model (publisher pushes data to subscribers) and Pull model (subscribers pull data from publisher). In this tutorial case, which variation of Observer Pattern that we use?**

**Answer:** Dalam tutorial ini, variasi pola desain Observer yang digunakan adalah Push Model. Pada implementasi ini, Publisher (aplikasi utama) secara aktif mengirimkan data notifikasi (payload) secara lengkap, seperti detail produk dan statusnya langsung kepada para Subscribers setiap kali terjadi sebuah event (pembuatan, promosi, atau penghapusan produk). Hal ini dibuktikan melalui eksekusi metode update() pada model Subscriber, di mana sistem melakukan HTTP POST request untuk mengirimkan data secara langsung ke URL webhook milik subscriber. Oleh karena itu, Subscriber bertindak pasif dan tidak perlu melakukan request tambahan untuk menarik (pull) data dari Publisher.

---

**2. What are the advantages and disadvantages of using the other variation of Observer Pattern for this tutorial case? (example: if you answer Q1 with Push, then imagine if we used Pull)**

**Answer:** Jika aplikasi BambangShop menggunakan variasi Pull Model, Publisher hanya akan mengirimkan notifikasi ringkas (misalnya hanya berupa event type dan id produk) tanpa menyertakan payload data yang lengkap. Selanjutnya, Subscriber yang harus secara aktif melakukan request (menarik data) ke Publisher untuk mendapatkan detail informasi tersebut. Berikut adalah analisis kelebihan dan kekurangannya untuk kasus ini:

Kelebihan (Advantages):

1. Payload awal yang dikirimkan oleh Publisher sangat kecil, sehingga menghemat bandwidth pada proses broadcasting notifikasi pertama.

2. Subscriber memiliki kendali penuh kapan mereka ingin menarik (pull) data detailnya. Jika Subscriber sedang mengalami high load, mereka bisa menunda penarikan data, atau bahkan mengabaikannya jika data tersebut dirasa tidak relevan bagi sistem mereka.

3. Publisher dapat menambahkan layar otorisasi (seperti validasi token) saat Subscriber mencoba menarik detail data, sehingga memastikan hanya Subscriber yang berhak yang bisa membaca detail produknya.

Kekurangan (Disadvantages):

1. Jika terdapat 1000 subscriber, Publisher tidak hanya mengirim 1000 ping notifikasi, tetapi sesaat kemudian Publisher akan langsung dihujani oleh 1000 HTTP GET request secara bersamaan dari para subscriber yang ingin menarik data. Hal ini dapat membebani server dan menyebabkan DDoS-like effect.

2. Harus ada penambahan logika ekstra di kedua sisi. Subscriber harus mengimplementasikan fungsi untuk melakukan fetching data tambahan, dan Publisher harus menyediakan API endpoint khusus (seperti GET /product/<id>) yang mampu menangani request massal.

3. Subscriber membutuhkan waktu yang lebih lama untuk mendapatkan informasi utuh karena harus melalui proses round-trip komunikasi dua kali (menerima ping lalu melakukan request detail).

---

**3. Explain what will happen to the program if we decide to not use multi-threading in the notification process.**

**Answer:** Jika kita memutuskan untuk tidak menggunakan multi-threading (seperti thread::spawn) dalam proses notifikasi, maka iterasi pengiriman HTTP request ke setiap subscriber akan berjalan secara sinkron dan sekuensial. Dampak utamanya adalah terjadinya I/O Blocking yang menyebabkan penurunan performa secara drastis (bottleneck). Waktu eksekusi program akan terakumulasi seiring dengan jumlah subscriber; misalnya, mengirim ke 1000 subscriber akan memakan waktu 1000 kali lipat dari waktu satu request. Selain itu, sistem menjadi tidak fault-tolerant. Jika salah satu URL subscriber mengalami timeout atau lambat merespon, proses iterasi akan tertahan, sehingga menunda pengiriman notifikasi ke subscriber lainnya. Dari sisi klien yang memicu event (misal: saat menambahkan produk), mereka akan mengalami latensi respon (waktu tunggu/loading) yang sangat lama karena thread utama tertahan oleh proses pengiriman notifikasi massal tersebut.