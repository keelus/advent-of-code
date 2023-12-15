package day04

import (
	"crypto/md5"
	"encoding/hex"
	"fmt"
)

func findMd5Hash(key string, searching string) int {
	iter := 0

	for {
		iter++
		key := fmt.Sprintf("%s%d", key, iter)

		md5Hash := md5.Sum([]byte(key))
		beginning := hex.EncodeToString(md5Hash[:])[:len(searching)]

		if beginning == searching {
			return iter
		}
	}

}
