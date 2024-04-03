# Variables

Değerleri Variable'larda Saklama" bölümünde belirtildiği gibi, varsayılan olarak variable'lar immutable'dır. Bu,
kodunuzu
Rust'ın sunduğu güvenlik ve kolay eşzamanlılıktan yararlanacak şekilde yazmanız için Rust'ın size verdiği birçok
dürtüden biridir. Ancak yine de değişkenlerinizi değiştirilebilir yapma seçeneğiniz vardır. Rust'ın sizi Immutableliği
tercih etmeye nasıl ve neden teşvik ettiğini ve bazen neden vazgeçmek isteyebileceğinizi inceleyelim.

````
fn main() {
    let x = 10;
    println!("The value of x : {x}");

    x = 11;
    println!("The value of x : {x}");
}
````

Bir değişken immutable olduğunda, bir değer bir isme bağlandığında, bu değeri değiştiremezsiniz. Yukarıda ki kod x'e
ikinci atamada hata verecektir

Bu örnek, derleyicinin programlarınızdaki hataları bulmanıza nasıl yardımcı olduğunu göstermektedir. Derleyici hataları
sinir bozucu olabilir, ancak aslında sadece programınızın henüz yapmasını istediğiniz şeyi güvenli bir şekilde yapmadığı
anlamına gelir; iyi bir programcı olmadığınız anlamına gelmez! Deneyimli Rustaceanlar hala derleyici hataları alırlar.

Immutable x değişkenine ikinci bir değer atamaya çalıştığınız için `x` immutable değişkenine iki kez atama yapılamaz
hata mesajını aldınız.

Immutable olarak belirlenmiş bir değeri değiştirmeye çalıştığımızda derleme zamanı hataları almamız önemlidir çünkü bu
durum hatalara yol açabilir. Kodumuzun bir bölümü bir değerin asla değişmeyeceği varsayımıyla çalışıyorsa ve kodumuzun
başka bir bölümü bu değeri değiştirirse, kodun ilk bölümünün yapmak için tasarlandığı şeyi yapmaması mümkündür. Bu tür
bir hatanın nedenini sonradan bulmak zor olabilir, özellikle de ikinci kod parçası değeri yalnızca bazen değiştiriyorsa.
Rust derleyicisi, bir değerin değişmeyeceğini belirttiğinizde, gerçekten değişmeyeceğini garanti eder, böylece bunu
kendiniz takip etmek zorunda kalmazsınız. Böylece kodunuzun mantık yürütmesi daha kolay olur.

Ancak değişebilirlik çok faydalı olabilir ve kod yazmayı daha kolay hale getirebilir. Değişkenler varsayılan olarak
Immutable olsa da, Bölüm 2'de yaptığınız gibi değişken adının önüne `mut` ekleyerek onları değişebilir hale
getirebilirsiniz. Mut eklemek ayrıca kodun diğer bölümlerinin bu değişkenin değerini değiştireceğini göstererek kodun
gelecekteki okuyucularına niyet iletir.

````
fn main() {
    let mut x = 10;
    println!("The value of x : {x}");

    x = 11;
    println!("The value of x : {x}");
}
````

Mut kullanıldığında x'e bağlı değeri 10'dan 11'e değiştirmemize izin verilir.

### Constants

Immutable değişkenler gibi, constant'lar da bir isme bağlı olan ve değişmesine izin verilmeyen değerlerdir, ancak
constantlar ve değişkenler arasında birkaç fark vardır.

İlk olarak, constantlar ile mut kullanmanıza izin verilmez. Constantlar yalnızca varsayılan olarak Immutable değildir,
her zaman Immutabledir. Constantları let anahtar sözcüğü yerine const anahtar sözcüğünü kullanarak bildirirsiniz ve
değerin türü ek açıklamalı olmalıdır. Türleri ve tür ek açıklamalarını bir sonraki bölüm olan "Veri Türleri" bölümünde
ele alacağız, bu nedenle şu anda ayrıntılar hakkında endişelenmeyin. Sadece her zaman türe açıklama eklemeniz
gerektiğini bilin.

Constant'lar, global kapsam da dahil olmak üzere herhangi bir kapsamda bildirilebilir, bu da onları kodun birçok
bölümünün bilmesi gereken değerler için kullanışlı kılar.

Son fark, constant'ların yalnızca runtime'da hesaplanabilecek bir değerin sonucuna değil, yalnızca sabit bir
Expressionye
ayarlanabilmesidir.

````
fn main() {
    const THREE_HOUR_IN_SECONDS : u32 = 60 * 60 * 3;
    println!("Three hours in seconds : {THREE_HOUR_IN_SECONDS}");
}
````

Constant'ın adı THREE_HOURS_IN_SECONDS'dur ve değeri 60 (bir dakikadaki saniye sayısı) ile 60'ın (bir saatteki dakika
sayısı) 3 (bu programda saymak istediğimiz saat sayısı) ile çarpımının sonucuna ayarlanır. Rust'ın constant'lar için
adlandırma kuralı, sözcükler arasında alt çizgi ile tüm büyük harfleri kullanmaktır. Derleyici, derleme zamanında
sınırlı bir Array işlemi değerlendirebilir, bu da bu sabiti 10.800 değerine ayarlamak yerine bu değeri anlaşılması ve
doğrulanması daha kolay bir şekilde yazmayı seçmemizi sağlar.

Constant'lar, bildirildikleri kapsam dahilinde bir programın çalıştığı süre boyunca geçerlidir. Bu özellik, uygulama
alanınızda programın birden fazla bölümünün bilmesi gerekebilecek değerler için Constant'lari kullanışlı hale getirir;
örneğin bir oyundaki herhangi bir oyuncunun kazanmasına izin verilen maksimum puan sayısı veya ışık hızı gibi.

### Shadowing

Bölüm 2'deki guessing game eğitiminde gördüğünüz gibi, önceki bir değişkenle aynı ada sahip yeni bir değişken
bildirebilirsiniz. Rustaceanlar ilk değişkenin ikincisi tarafından gölgelendiğini söyler, bu da değişkenin adını
kullandığınızda derleyicinin göreceği şeyin ikinci değişken olduğu anlamına gelir. Gerçekte, ikinci değişken birinciyi
gölgeler ve kendisi gölgelenene ya da kapsam sona erene kadar değişken adının tüm kullanımlarını kendine alır. Aynı
değişkenin adını kullanarak ve let anahtar sözcüğünün kullanımını aşağıdaki gibi tekrarlayarak bir değişkeni
gölgeleyebiliriz:

````
fn main() {
    let x = 5;
    let x = x + 1;

    {
        /* x scope disinda 6 degerine shadowing edildi */
        let x = x * 2;
        println!("The value of x in the inner scope is : {x}"); // 12 sonucu return edilir
    }

    println!("The value of x is : {x}"); // 6 sonucu return edilir
}
````

Bu program ilk olarak x'i 5 değerine bağlar. Daha sonra let x = Expressionsini tekrarlayarak yeni bir x değişkeni
oluşturur,
orijinal değeri alır ve 1 ekler, böylece x'in değeri 6 olur. Ardından, küme parantezleriyle oluşturulan bir iç kapsam
içinde, üçüncü let Statementi de x'i gölgeler ve x'e 12 değerini vermek için önceki değeri 2 ile çarparak yeni bir
değişken
oluşturur. Bu kapsam sona erdiğinde, iç gölgeleme sona erer ve x 6 değerine geri döner. Bu programı çalıştırdığımızda,
aşağıdaki çıktıyı verecektir:

![img.png](img.png)

Gölgeleme, bir değişkeni mut olarak işaretlemekten farklıdır çünkü let anahtar sözcüğünü kullanmadan yanlışlıkla bu
değişkene yeniden atama yapmaya çalışırsak derleme zamanı hatası alırız. let kullanarak, bir değer üzerinde birkaç
dönüşüm gerçekleştirebilir, ancak bu dönüşümler tamamlandıktan sonra değişkenin immutable olmasını sağlayabiliriz.

Mut ve gölgeleme arasındaki diğer fark, let anahtar sözcüğünü tekrar kullandığımızda etkin bir şekilde yeni bir değişken
oluşturduğumuz için, değerin türünü değiştirebilir ancak aynı adı yeniden kullanabiliriz. Örneğin, programımızın bir
kullanıcıdan boşluk karakterleri girerek bazı metinler arasında kaç boşluk istediğini göstermesini istediğini ve
ardından bu girdiyi bir sayı olarak saklamak istediğimizi varsayalım:

````
fn main() {
    let spaces = "     ";
    let spaces = spaces.len();
    println!("Spaces length : {spaces}");
}
````

İlk spaces değişkeni bir string tipidir ve ikinci spaces değişkeni bir sayı tipidir. Böylece gölgeleme bizi spaces_str
ve spaces_num gibi farklı isimler bulmaktan kurtarır; bunun yerine daha basit olan spaces ismini tekrar kullanabiliriz.
Ancak, burada gösterildiği gibi bunun için mut kullanmaya çalışırsak, derleme zamanı hatası alırız:

````
fn main() {
    let mut spaces = "     ";
    spaces = spaces.len();
    println!("Spaces length : {spaces}");
}
````

![img_1.png](img_1.png)

Hata, bir değişkenin türünü değiştirmemize izin verilmediğini söylüyor: