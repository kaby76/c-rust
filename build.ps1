# Generated from trgen 0.23.27

$version="4.13.1"
$env:GO111MODULE = "on"
For ($i=0; $i -le 5; $i++) {
	$(& go get github.com/antlr4-go/antlr/v4@v$version ; $compile_exit_code = $LASTEXITCODE) | Write-Host
	if($compile_exit_code -eq 0){
		Break
	}
	Write-Host "go get failed. Trying again."
}
if($compile_exit_code -ne 0){
    exit $compile_exit_code
}
For ($i=0; $i -le 5; $i++) {
	$(& go get golang.org/x/text@v0.26.0 ; $compile_exit_code = $LASTEXITCODE) | Write-Host
	if($compile_exit_code -eq 0){
		Break
	}
	Write-Host "go get failed. Trying again."
}
if($compile_exit_code -ne 0){
    exit $compile_exit_code
}

if (Test-Path -Path transformGrammar.py -PathType Leaf) {
    $(& python3 transformGrammar.py ) 2>&1 | Write-Host
}

$(& antlr4 -v $version C.g4 -encoding utf-8 -Dlanguage=Go -o parser -lib parser -package parser     ; $compile_exit_code = $LASTEXITCODE) | Write-Host
if($compile_exit_code -ne 0){
    exit $compile_exit_code
}
$(& antlr4 -v $version C.g4 -encoding utf-8 -Dlanguage=Go -o parser -lib parser -package parser     ; $compile_exit_code = $LASTEXITCODE) | Write-Host
if($compile_exit_code -ne 0){
    exit $compile_exit_code
}


$(& go build ; $compile_exit_code = $LASTEXITCODE) | Write-Host
exit $compile_exit_code
