# Capibara
Capibara is a cli tool for making basic GET/POST requests.
TMI: Capibara is an extension of urlChecker which is removed repo.


> [!NOTE]
> I'm not Capybara! I'm Capi!bara!
>
> output as file might be or not supported

## example
to make basic request(s)
```
    ./capibara.exe -u https://your_target.com ...(other url(s))
```
if you want to see your result as body/status, then use -b with true. If not then false
```
    ./capibara.exe -u https://your_target.com -b true
```
if you want to send the request(s) by using a input file
```
    ./capibara.exe -i /path/to/urls.txt
```
other features are currently under development


### todo
1. implement output argument
2. [Option] consider using tokio_scoped crate to solve 
error[E0521]: borrowed data escapes outside of method
   --> src\app.rs:95:24
    |
 88 |       async fn run_out_as_body(&self) {
    |                                -----
    |                                |
    |                                `self` is a reference that is only valid in the method body
    |                                let's call the lifetime of this reference `'1`
...
 95 |               t_vec.push(tokio::spawn(async move {
    |  ________________________^
 96 | |                 match req.request(&url, &method).await{
 97 | |                     Ok(req) => println!("url: {}\nbody:\n{}", &url, req.body),
 98 | |                     Err(e) => eprintln!("url: {} {:?}", &url, e)
 99 | |                 }
100 | |             }));
    | |              ^
    | |              |
    | |______________`self` escapes the method body here
    |                argument requires that `'1` must outlive `'static`
since scoped spawn does not require 'static unlike tokio::spawn
3. add tests
