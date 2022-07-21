#!/bin/bash

set -euxo pipefail

uid=$(uuidgen | tr '[:upper:]' '[:lower:]')
iss=$(date +"%Y-%m-%d")

wget -O "$uid.pdf" "$LASTEST_URL"

cat <<EOF >>${uid}.metadata
{
    "deleted": false,
    "lastModified": "$(date +%s)000",
    "metadatamodified": false,
    "modified": false,
    "parent": "",
    "pinned": false,
    "synced": false,
    "type": "DocumentType",
    "version": 1,
    "visibleName": "Rzepa $iss"
}
EOF

cat <<EOF >${uid}.content
{
    "extraMetadata": {
    },
    "fileType": "pdf",
    "fontName": "",
    "lastOpenedPage": 0,
    "lineHeight": -1,
    "margins": 100,
    "pageCount": 1,
    "textScale": 1,
    "transform": {
        "m11": 1,
        "m12": 1,
        "m13": 1,
        "m21": 1,
        "m22": 1,
        "m23": 1,
        "m31": 1,
        "m32": 1,
        "m33": 1
    }
}
EOF

mkdir "${uid}.cache"
mkdir "${uid}.highlights"
mkdir "${uid}.thumbnails"

mv --verbose ${uid}.* ~/.local/share/remarkable/xochitl

systemctl restart xochitl

echo $uid done

