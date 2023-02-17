link=https://at.alicdn.com/t/c/font_3901890_ltmq3yzqdb.js

curl ${link} -o ./src/icons/icons.js
echo -e "// eslint-disable-next-line\n$(cat ./src/icons/icons.js)" > ./src/icons/icons.js
