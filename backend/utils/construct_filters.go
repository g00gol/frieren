package utils

import (
	"net/http"
	"reflect"
	"strings"

	"go.mongodb.org/mongo-driver/bson"
)

func ConstructFilters(r *http.Request, model interface{}) bson.D {
	val := reflect.ValueOf(model)
	typ := val.Type()

	filter := bson.D{}

	for i := 0; i < typ.NumField(); i++ {
		field := typ.Field(i)
		param := strings.ToLower(field.Name)

		if values, ok := r.URL.Query()[param]; ok && len(values) > 0 {
			if field.Type.Kind() == reflect.Slice {
				// Split the comma-delimited string into a slice of strings
				elements := strings.Split(values[0], ",")
				filter = append(filter, bson.E{Key: param, Value: bson.D{{Key: "$in", Value: elements}}})
			} else {
				filter = append(filter, bson.E{Key: param, Value: values[0]})
			}
		}
	}

	return filter
}
