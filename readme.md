Rust, varsayılan olarak standart kütüphanede tanımlanan ve her programın kapsamına giren bir dizi öğeye sahiptir. Bu
kümeye "prelude" denir ve içindeki her şeyi standart kütüphane belgelerinde görebilirsiniz.

https://doc.rust-lang.org/std/prelude/index.html

Random kütüphanesi rust'ın standart kütüphanesinde olmadığı için Cargo.toml dosyası içerisine ilgili dependency'leri
eklememiz gerekiyor

cargo.toml içerisine Random dependencies'lerini ekledikten sonra cargo build komutu ile projeyi build etmeliyiz

Harici bir bağımlılık eklediğimizde, Cargo bu bağımlılığın ihtiyaç duyduğu her şeyin en son sürümlerini Crates.io'daki
verilerin bir kopyası olan kayıt defterinden alır. Crates.io, Rust ekosistemindeki insanların açık kaynaklı Rust
projelerini başkalarının kullanması için yayınladıkları yerdir.

Kayıt defterini güncelledikten sonra, Cargo [Dependencies] bölümünü kontrol eder ve daha önce indirilmemiş olan tüm
crate'leri indirir. Bu durumda, sadece rand'ı bağımlılık olarak listelemiş olsak da, Cargo rand'ın çalışmak için bağımlı
olduğu diğer crate'leri de yakaladı. Crate'leri indirdikten sonra, Rust bunları derler ve ardından projeyi mevcut
bağımlılıklarla derler.

Cargo, siz veya bir başkası kodunuzu her derlediğinde aynı eseri yeniden oluşturabilmenizi sağlayan bir mekanizmaya
sahiptir: Cargo, siz aksini belirtmedikçe yalnızca belirttiğiniz bağımlılıkların sürümlerini kullanacaktır. Örneğin,
gelecek hafta rand crate'inin 0.8.6 sürümünün çıktığını ve bu sürümün önemli bir hata düzeltmesi içerdiğini, ancak aynı
zamanda kodunuzu bozacak bir regresyon içerdiğini varsayalım. Bunu ele almak için Rust, cargo derlemesini ilk kez
çalıştırdığınızda Cargo.lock dosyasını oluşturur, bu nedenle şimdi bunu guessing_game dizininde bulunduruyoruz.

Bir projeyi ilk kez oluşturduğunuzda, Cargo kriterlere uyan bağımlılıkların tüm sürümlerini belirler ve ardından bunları
Cargo.lock dosyasına yazar. Gelecekte projenizi derlediğinizde, Cargo Cargo.lock dosyasının var olduğunu görecek ve
sürümleri tekrar bulmak için tüm işi yapmak yerine orada belirtilen sürümleri kullanacaktır. Bu, otomatik olarak yeniden
üretilebilir bir yapıya sahip olmanızı sağlar. Başka bir deyişle, Cargo.lock dosyası sayesinde siz açıkça yükseltme
yapana kadar projeniz 0.8.5 sürümünde kalacaktır. Cargo.lock dosyası tekrarlanabilir derlemeler için önemli olduğundan,
genellikle projenizdeki kodun geri kalanıyla birlikte kaynak kontrolünde kontrol edilir.

Bir crate'i güncellemek istediğinizde, Cargo, Cargo.lock dosyasını yok sayacak ve Cargo.toml'daki spesifikasyonlarınıza
uyan en son sürümleri bulacak olan update komutunu sağlar. Cargo daha sonra bu sürümleri Cargo.lock dosyasına
yazacaktır. Aksi takdirde, varsayılan olarak, Cargo yalnızca 0.8.5'ten büyük ve 0.9.0'dan küçük sürümleri arayacaktır.
Eğer rand crate iki yeni 0.8.6 ve 0.9.0 sürümlerini yayınladıysa, cargo update'i çalıştırdığınızda aşağıdakileri
görürsünüz:

![img.png](guessing_game/img.png)

Cargo 0.9.0 sürümünü yok sayar. Bu noktada, Cargo.lock dosyanızda şu anda kullandığınız rand crate sürümünün 0.8.5
olduğunu belirten bir değişiklik de fark edeceksiniz. Rand'ın 0.9.0 sürümünü ya da 0.9.x serisindeki herhangi bir sürümü
kullanmak için Cargo.toml dosyasını aşağıdaki gibi güncellemeniz gerekir:

![img_1.png](guessing_game/img_1.png)

Cargo build'i bir sonraki çalıştırışınızda, Cargo mevcut crate'lerinin kayıt defterini güncelleyecek ve rand
gereksinimlerinizi belirttiğiniz yeni sürüme göre yeniden değerlendirecektir.

Bir crate'de hangi özelliklerin kullanılacağını ve hangi method ve işlevlerin çağrılacağını bilmeniz yeterli
olmayacaktır, bu nedenle her crate, kullanım talimatlarını içeren belgelere sahiptir. Cargo'nun bir başka güzel
özelliği de cargo doc --open komutunu çalıştırdığınızda tüm bağımlılıklarınız tarafından sağlanan belgeleri yerel olarak
oluşturması ve tarayıcınızda açmasıdır. Örneğin rand crate'indeki diğer işlevlerle ilgileniyorsanız, cargo doc --open
komutunu çalıştırın ve soldaki kenar çubuğunda rand'a tıklayın.