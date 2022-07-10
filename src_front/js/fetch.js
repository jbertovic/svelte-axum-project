

export async function getSecure() {
    let res = await fetch('/secure',{credentials: 'same-origin'});
    let secureResponse = await res.json();
    return JSON.stringify(secureResponse.session);
} 
