# rules

### exif-has-capture-datetime

guards presence and validity of capture date and time

| option          | type                         | required | description                                      |
| --------------- | ---------------------------- | -------- | ------------------------------------------------ |
| `date`          | `boolean \| null`            | `no`     | require capture date                             |
| `level`         | `"off" \| "warn" \| "error"` | `no`     | severity level (default: warn)                   |
| `min_year`      | `integer \| null`            | `no`     | minimum allowed year for capture date            |
| `reject_future` | `boolean \| null`            | `no`     | reject capture dates later than the current date |
| `time`          | `boolean \| null`            | `no`     | require capture time                             |

### exif-has-coords

guards presence of gps coordinates

| option      | type                         | required | description                    |
| ----------- | ---------------------------- | -------- | ------------------------------ |
| `latitude`  | `boolean \| null`            | `no`     | require gps latitude           |
| `level`     | `"off" \| "warn" \| "error"` | `no`     | severity level (default: warn) |
| `longitude` | `boolean`                    | `yes`    | require gps longitude          |

### fs-broken-symlink

guards that symbolic links resolve to existing targets

| option  | type                         | required | description                    |
| ------- | ---------------------------- | -------- | ------------------------------ |
| `level` | `"off" \| "warn" \| "error"` | `no`     | severity level (default: warn) |

### fs-permission

guards file permission mode, owner, and group

| option  | type                         | required | description                                                   |
| ------- | ---------------------------- | -------- | ------------------------------------------------------------- |
| `group` | `string \| null`             | `no`     | enforce specific group                                        |
| `level` | `"off" \| "warn" \| "error"` | `no`     | severity level (default: warn)                                |
| `mode`  | `string \| null`             | `no`     | enforce mode (octal or symbolic, e.g. "0644'" or "rw-r--r--") |
| `owner` | `string \| null`             | `no`     | enforce specific owner                                        |
