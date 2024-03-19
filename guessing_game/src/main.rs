use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    /* println! bir String'i ekrana yazdıran bir makrodur */
    println!("Guess the number");

    /* İlk olarak use rand::Rng; satırını ekliyoruz. Rng özelliği, rastgele sayı üreticilerinin
    implement ettiği methodları tanımlar ve bu methodları kullanabilmemiz için bu özelliğin kapsam
    dahilinde olması gerekir. Ardından mevcut thread'e ait gen_range methodu ile 1 ila 100 arasında
    random bir sayı üretiriz. Eğer bu dependencies (rand) ile ilgili dökümana ihtiyacınız varsa
    komut satırında cargo doc --open dediğinizde library'nin kullanımı ile ilgili tüm dökümantasyon
    otomatik olarak download edilir*/
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Secret number is : {}", secret_number);

    loop {
        /* Ardından, kullanıcı inputunu saklamak için aşağıdaki gibi bir değişken oluşturacağız
           Değişkeni oluşturmak için let deyimini kullanıyoruz
           Rust'ta değişkenler varsayılan olarak immutable'dır, yani değişkene bir değer verdiğimizde değer değişmez.
           Bir değişkeni mutable (değiştirilebilir) yapmak için değişken adının önüne mut ekleriz:

           Eşittir işaretinin sağında, guess'in bağlı olduğu değer yer alır; bu değer, String'in yeni
           bir örneğini döndüren bir fonksiyon olan String::new çağrısının sonucudur. String,
           standart kütüphane tarafından sağlanan, büyüyebilen, UTF-8 kodlu bir metin parçası olan bir
           dize türüdür.

           ::new satırındaki :: sözdizimi, new öğesinin String türünün ilişkili bir işlevi olduğunu
           gösterir. İlişkili fonksiyon, bir tür üzerinde (bu durumda String) uygulanan bir fonksiyondur.
           Bu new fonksiyonu yeni, boş bir String oluşturur. new fonksiyonunu birçok türde bulabilirsiniz,
           çünkü bir tür yeni değer oluşturan bir fonksiyonun ortak adıdır.
        */
        let mut guess = String::new();

        /* io kütüphanesi, std olarak bilinen standart kütüphaneden gelir */
        /* Kullanmak istediğiniz bir tür başlangıçta yoksa, o türü bir use deyimiyle açıkça kapsama
        almanız gerekir. std::io kütüphanesini kullanmak size kullanıcı girdisini kabul etme yeteneği de
        dahil olmak üzere bir dizi yararlı özellik sağlar */
        /* io modülünden stdin fonksiyonunu çağıracağız, bu da kullanıcı girdisini işlememizi
        sağlayacak. Programın başında use std::io; ile io kütüphanesini içe aktarmamış olsaydık,
        bu fonksiyon çağrısını std::io::stdin olarak yazarak fonksiyonu yine de kullanabilirdik. */
        io::stdin()
            /* kullanıcı inputunu hangi dizede saklayacağını söylemek için read_line'a argüman olarak
            &mut guess'i geçiyoruz. read_line'ın tam görevi, kullanıcının standart inputa yazdığı
            her şeyi almak ve bunu bir dizeye eklemektir (içeriğinin üzerine yazmadan), bu nedenle bu
            dizeyi bir argüman olarak geçiyoruz. String argümanının mutable olması gerekir, böylece
            metot stringin içeriğini değiştirebilir. Bu argümanın bir referans olduğunu belirten & işareti,
            kodunuzun birden fazla bölümünün bir veri parçasına, bu veriyi birden fazla kez belleğe
            kopyalamak zorunda kalmadan erişmesini sağlar. Referanslar karmaşık bir özelliktir ve Rust'ın
            en büyük avantajlarından biri referansları kullanmanın ne kadar güvenli ve kolay olduğudur.
            Bu programı bitirmek için bu detayların çoğunu bilmenize gerek yok. Şimdilik bilmeniz gereken
            tek şey, değişkenler gibi referansların da varsayılan olarak immutable olduğudur. Bu nedenle,
            değiştirilebilir yapmak için &guess yerine &mut guess yazmanız gerekir.*/
            .read_line(&mut guess)
            /* Daha önce de belirtildiği gibi, read_line kullanıcının girdiği her şeyi kendisine
            ilettiğimiz String'e koyar, ancak aynı zamanda bir Result değeri de döndürür. Result,
            genellikle enum olarak adlandırılan ve birden fazla olası state'den birinde olabilen bir tür
            olan bir eumeration'dır. Her olası state'e bir varyant diyoruz. Result'ın varyantları
            Ok ve Err'dir. Ok değişkeni işlemin başarılı olduğunu gösterir ve Ok içinde başarıyla
            oluşturulan değer bulunur. Err değişkeni, işlemin başarısız olduğu anlamına gelir ve Err,
            işlemin nasıl veya neden başarısız olduğu hakkında bilgi içerir. Result türündeki değerler,
            diğer türlerdeki değerler gibi, üzerlerinde tanımlanmış methodlara sahiptir. Bir Result
            örneğinin çağırabileceğiniz bir expect yöntemi vardır. Bu Result örneği bir Err değeriyse,
            expect programın çökmesine neden olur ve expect'e argüman olarak aktardığınız mesajı görüntüler.
            read_line methodu bir Err döndürürse, bu büyük olasılıkla temel işletim sisteminden gelen
            bir hatanın sonucu olacaktır. Bu Result örneği bir Ok değeriyse, expect Ok'un tuttuğu dönüş
            değerini alır ve kullanabilmeniz için size yalnızca bu değeri döndürür. Bu durumda, bu değer
            kullanıcının inputunda ki bayt sayısıdır. Eğer expect'i çağırmazsanız, program derlenir,
            ancak bir uyarı alırsınız. Rust, read_line'dan dönen Result değerini kullanmadığınız
            konusunda uyararak programın olası bir hatayı ele almadığını belirtir. Uyarıyı bastırmanın
            doğru yolu aslında error handling kodu yazmaktır, ancak bizim durumumuzda sadece bir sorun
            oluştuğunda bu programı çökertmek istiyoruz, bu yüzden expect'i kullanabiliriz. */
            .expect("Failed to read line");

        /* guess normalde string olarak tanımlanmış bir değişkendir. Fakat biz random bir secret_number
        generate ederken tipik olarak i32 tipinde bir deger üretecektir. Dolayısıyla aşağıdaki match
        fonksiyonunda cmp özelliğini kullanırken değerlerin birbirlerinin aynı tipinde olması
        gerekmektedir. Programın zaten guess adında bir değişkeni yok mu? Var, ancak Rust bize guess'in
        önceki değerini yeni bir değerle shadow etmemize izin veriyor. Shadowing, örneğin guess_str ve guess
        gibi iki benzersiz değişken oluşturmaya zorlamak yerine guess değişken adını yeniden kullanmamızı
        sağlar. Bu yeni değişkeni guess.trim().parse() ifadesine bağlarız. İfadedeki guess, inputu bir
        String olarak içeren orijinal guess değişkenini ifade eder. String örneğindeki trim yöntemi,
        başlangıç ve sondaki tüm boşlukları ortadan kaldırır; bunu, String'i yalnızca sayısal veriler
        içerebilen u32 ile karşılaştırabilmek için yapmamız gerekir. Kullanıcı read_line komutunu yerine
        getirmek için enter tuşuna basmalı ve tahminini girmelidir, bu da String'e bir satırsonu
        karakteri ekler. Örneğin, kullanıcı 5 yazıp enter tuşuna basarsa, tahmin şu şekilde görünür: 5\n.
        Buradaki \n "yeni satır "ı temsil eder. (Windows'ta enter tuşuna basıldığında satır başı ve
        satırsonu karakteri eklenir, \r\n.) Trim methodu \n veya \r\n'yi ortadan kaldırarak yalnızca 5
        sonucunu verir. String'ler üzerindeki parse methodu, bir String'i başka bir türe dönüştürür.
        Burada, bir String'den bir sayıya dönüştürmek için kullanıyoruz. Rust'a let guess: u32 kullanarak
        istediğimiz tam sayı türünü söylememiz gerekir. guess'ten sonraki iki nokta üst üste (:) Rust'a
        değişkenin türüne açıklama ekleyeceğimizi söyler. Rust'ın birkaç yerleşik sayı türü vardır;
        burada görülen u32 işaretsiz, 32 bitlik bir tamsayıdır. Parse methodu yalnızca mantıksal olarak
        sayıya dönüştürülebilen karakterler üzerinde çalışır ve bu nedenle kolayca hatalara neden olabilir.
        Örneğin, String A��% gibi karakterler içeriyorsa, bunu sayıya dönüştürmenin bir yolu yoktur.
        Başarısız olabileceği için, parse yöntemi, read_line yönteminin yaptığı gibi bir Result türü
        döndürür. Bu Result'a yine expect methodunu kullanarak aynı şekilde davranacağız. Eğer parse,
        String'den bir sayı oluşturamadığı için bir Err Result çeşidi döndürürse, expect çağrısı oyunu
        çökertecek ve verdiğimiz mesajı yazdıracaktır. Eğer parse String'i başarılı bir şekilde sayıya
        dönüştürebilirse, Result'un Ok değişkenini döndürecektir ve expect, Ok değerinden istediğimiz
        sayıyı döndürecektir. Kullanıcı u32 bir değer girmediğinde programımız çöküyor du bunun önüne
        geçmek için expect ifadesini silip kodu değiştiriyoruz*/
        //let guess : u32 = guess.trim().parse().expect("Please type a number");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            /* Alt çizgi, _, bir catchall değeridir; bu örnekte, içlerinde hangi bilgi olursa olsun
            tüm Err değerlerini eşleştirmek istediğimizi söylüyoruz*/
            Err(_) => continue,
        };

        /* Bu satır, kullanıcının inputunu içeren String'i yazdırır. Küme parantezlerinin {} kümesi bir
        yer tutucudur: {}'i bir değeri yerinde tutan küçük yengeç kıskaçları olarak düşünün. Bir değişkenin
        değerini yazdırırken, değişken adı küme parantezlerinin içine girebilir. Bir ifadenin
        değerlendirilmesinin sonucunu yazdırırken, biçim dizesine boş küme parantezleri yerleştirin,
        ardından biçim dizesini her boş küme parantezi yer tutucusuna aynı sırada yazdırılacak ifadelerin
        virgülle ayrılmış bir listesiyle takip edin.*/
        println!("Your guessed : {}", guess);

        /* İlk olarak, std::cmp::Ordering adlı bir türü standart kütüphaneden kapsam içine alarak başka
        bir use deyimi ekliyoruz. Ordering türü başka bir enum'dur ve Less, Greater ve Equal
        enumlarına sahiptir. Bunlar, iki değeri karşılaştırdığınızda mümkün olan üç sonuçtur */
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                /* You win!'den sonra break satırının eklenmesi, kullanıcı gizli sayıyı doğru
                tahmin ettiğinde programın döngüden çıkmasını sağlar. Döngüden çıkmak aynı zamanda
                programdan çıkmak anlamına gelir, çünkü döngü main'in son parçasıdır.*/
                println!("You win!");
                break;
            }
        }
    }
}

