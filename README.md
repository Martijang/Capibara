# Capibara
Capibara is a cli tool for making basic GET/POST requests.

TMI: Capibara is an extension of urlChecker. Which is currently private repo.


> [!NOTE]
> I'm not Capybara! I'm Capi!bara!
>
> requesting with body is still experimental which means its not tested yet

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
for more infomation run ./capibara.exe -help


### todo
1. add tests
