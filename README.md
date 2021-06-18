[![GitHub license](https://img.shields.io/github/license/Naereen/StrapDown.js.svg)](https://github.com/Naereen/StrapDown.js/blob/master/LICENSE)
<br>

# ColorCurl

colorcurl (ccurl) is a CLI utlity that pretty print colored http calls and rest api pulls.

## Installation

Just download the executable for your platform and run it from the console.
Example rest api call:<br>
cd into the same folder of the executable:
```bash
./ccurl -r http://api_path_to_be_called/resource/1
```
Example response:

```json
{
  "foo" : "foo_foo",
  "bar" : "bar_bar"
}
```

## Usage:
Parameters:

```bash
-r : REST api call
-h : print help-message

```

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License
[MIT](https://choosealicense.com/licenses/mit/)
