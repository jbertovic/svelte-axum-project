import {user} from './js/store.js';

async function fetch_session() {
    const res = await fetch('/auth/session',{credentials: 'same-origin'});
    let sessionResponse = await res.json();
    if (sessionResponse.user_id !== '') {
        user.set(sessionResponse.user_id);
    } else
    {
        user.set('');
    }
}

module.exports = {fetch_session};