#/usr/bin/env sh

if [[ "$PWD" == */.github ]]; then
	RESTORE_PWD=true
	pushd .. > /dev/null
fi

echo "----- Files in violation -----"
find src/ -name '*.rs' -type f -exec wc -l {} \; | rg '(?:(?:\d+[0-9])|[5-9])\d{2}'

echo "----- Test Files in violation -----"
find tests/ -name '*.rs' -type f -exec wc -l {} \; | rg '(?:(?:\d+[0-9])|[5-9])\d{2}'

echo "----- Functions in violation -----"
rg -iHUPN --multiline-dotall -g '*.rs' -e '^\t*(?:pub )?fn.*?^\t*}\n\n(?=\t*[\/pfei#]|\z)' src/ -r '$0\n---FUNC---' | awk -v RS='---FUNC---' '{n=split($0,a,"\n"); if(a[1]!="") print n-1, a[1]}' | rg '^\w*(?:(?:\d+[0-9])|[5-9])\d{1}'

echo "----- Test Functions in violation -----"
rg -iHUPN --multiline-dotall -g '*.rs' -e '^\t*(?:pub )?fn.*?^\t*}\n\n(?=\t*[\/pfei#]|\z)' tests/ -r '$0\n---FUNC---' | awk -v RS='---FUNC---' '{n=split($0,a,"\n"); if(a[1]!="") print n-1, a[1]}' | rg '^\w*(?:(?:\d+[0-9])|[5-9])\d{1}'

echo "----- Short Functions -----"
rg -iHUPN --multiline-dotall -g '*.rs' -e '^(?:\t*(?:pub )?fn\w*).*?(?:^\t*}\n\n(?=\t*[\/pfei#]|\z))' src/ -r '$0\n---FUNC---' | awk -v RS='---FUNC---' '{n=split($0,a,"\n"); if(a[1]!="") print n-1, a[1]}' | rg '^[^0-9]'

if [[ "$RESTORE_PWD" == true ]]; then
	popd > /dev/null
fi

exit 0
