/*
The client can send two possible kinds of messages:
- sendCommand: sends a specified REST command
- getPage: requests HTML code that will be assigned to the body
*/
function sendCommand(method, uri, body, success, failure) {
    var xhttp = new XMLHttpRequest();
    xhttp.onreadystatechange = function() {
        if (this.readyState == 4)
            if (this.status == 200) success();
            else failure();
    };
    xhttp.open(method, uri, true);
    xhttp.send(body);
}

function getPage(uri) {
    var xhttp = new XMLHttpRequest();
    xhttp.onreadystatechange = function() {
        if (this.readyState == 4 && this.status == 200) {
            document.getElementById('body')
                .innerHTML = xhttp.responseText;
        }
    };
    xhttp.open('GET', uri, true);
    xhttp.send();
}

function delete_selected_persons() {
    var items;
    for (var item of document.getElementsByName('selector'))
        if (item.checked)
            if (items) items += ',' + item.id;
            else items = '' + item.id;
    if (items)
        sendCommand('DELETE', '/persons?id_list=' + items, '',
            function() { getPage('/page/persons'); },
            function() { alert('Failed deletion.'); });
}

function savePerson(method) {
    sendCommand(method,
        '/one_person?'
        + (method === 'POST' ? '' :
        'id='
        + document.getElementById('person_id').value
        + '&')
        + 'name='
        + encodeURIComponent(
            document.getElementById('person_name')
            .value),
        '',
        function() {
            getPage('/page/persons');
        },
        function() {
            alert('Failed command.');
        });
}
