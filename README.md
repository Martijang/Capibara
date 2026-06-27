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
2. [Option] rewrite run_out_as_body/_as_status function if code is too massy
3. add tests
