<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Update Nama</description>
   <name>APITCCP_P_002_UpdateProfile</name>
   <tag></tag>
   <elementGuidId>1f8f217b-1e32-41a4-82fc-30a9dd213bbe</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIzIiwianRpIjoiY2QwZjMxMzg0NzIxNGQ4YzdlM2M4N2JhMGQwNDhjM2MxOThiZTk5MGViNjAyN2NmMDFmMTFjZTQ3NzVmYmUzMjczNTRkOGVlYjIzZWM3NmMiLCJpYXQiOjE2ODY4MDE2MzQuNTQ4ODY2LCJuYmYiOjE2ODY4MDE2MzQuNTQ4ODczLCJleHAiOjE3MTg0MjQwMzQuNTQ1MjA3LCJzdWIiOiI2NCIsInNjb3BlcyI6W119.LD4SUwFvZPR1shHsoFgilJIRO0BFCBXhiyG3rMrCg6t9ElcotNaR_i-M2KZKMnHk31z5Lerf4V0rDoMxIzgdp8VLqkk3AsN2SUdppJxtBwu9MuixyMLLIi4tXZTOrpUj_8Lzielyej4KM1IpLgzMawCRP4EilFrTyuCqfzsDbeEhYSEXqE0HyuNWN0J_bwiKGCYrBLKYclVXnPcyDAZetdnjEOSE2t-DAny9VFG7CX1LpE7wkX5K9S_IhzCFAUWD__ev71iuJYA5OFLesCkfPDKg1MbYulqRq3H0YMEbS2NxPT7n-7hVOzU5N_7ZdunU_nB3lyvlxo63_Ag4HppfrWsUUm5jzYAggZZvdk9qE9nNeOTGJzDKAV_Nv3vjDhlMdCR9bbfU18T1i1Y84KGBlGYGJzbZ1OSE_Mr2arkI2OnauQ3hftEvdGRDkApiQzSYa2QkNf3Hxm33P46w1gNU-Ie6CIgHKjWXqymp0LqnkY8kA5cVSntaSMcVbNfN8FewvGNYOvPfRQ2Py48sx09NW5xGr4xb23s2sznZ8m8WBpjNM6Ya7tpihdJicQE8uGNzq-dNNqR2SjBzqKOHSW2ld2trb0Aq7qnDuXSlCaOKUqsq9DiPMwQ4cTYetR17Ic-pQxCqIgELt46JdbUnLQsng3gBXkEXE0NK-VJCRuAk2A4</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;application/x-www-form-urlencoded&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;name&quot;,
      &quot;value&quot;: &quot;asep&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>x-www-form-urlencoded</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/x-www-form-urlencoded</value>
      <webElementGuid>0531a40c-f901-4422-bdf8-be2010327932</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIzIiwianRpIjoiY2QwZjMxMzg0NzIxNGQ4YzdlM2M4N2JhMGQwNDhjM2MxOThiZTk5MGViNjAyN2NmMDFmMTFjZTQ3NzVmYmUzMjczNTRkOGVlYjIzZWM3NmMiLCJpYXQiOjE2ODY4MDE2MzQuNTQ4ODY2LCJuYmYiOjE2ODY4MDE2MzQuNTQ4ODczLCJleHAiOjE3MTg0MjQwMzQuNTQ1MjA3LCJzdWIiOiI2NCIsInNjb3BlcyI6W119.LD4SUwFvZPR1shHsoFgilJIRO0BFCBXhiyG3rMrCg6t9ElcotNaR_i-M2KZKMnHk31z5Lerf4V0rDoMxIzgdp8VLqkk3AsN2SUdppJxtBwu9MuixyMLLIi4tXZTOrpUj_8Lzielyej4KM1IpLgzMawCRP4EilFrTyuCqfzsDbeEhYSEXqE0HyuNWN0J_bwiKGCYrBLKYclVXnPcyDAZetdnjEOSE2t-DAny9VFG7CX1LpE7wkX5K9S_IhzCFAUWD__ev71iuJYA5OFLesCkfPDKg1MbYulqRq3H0YMEbS2NxPT7n-7hVOzU5N_7ZdunU_nB3lyvlxo63_Ag4HppfrWsUUm5jzYAggZZvdk9qE9nNeOTGJzDKAV_Nv3vjDhlMdCR9bbfU18T1i1Y84KGBlGYGJzbZ1OSE_Mr2arkI2OnauQ3hftEvdGRDkApiQzSYa2QkNf3Hxm33P46w1gNU-Ie6CIgHKjWXqymp0LqnkY8kA5cVSntaSMcVbNfN8FewvGNYOvPfRQ2Py48sx09NW5xGr4xb23s2sznZ8m8WBpjNM6Ya7tpihdJicQE8uGNzq-dNNqR2SjBzqKOHSW2ld2trb0Aq7qnDuXSlCaOKUqsq9DiPMwQ4cTYetR17Ic-pQxCqIgELt46JdbUnLQsng3gBXkEXE0NK-VJCRuAk2A4</value>
      <webElementGuid>38fdd364-7071-405e-b5d2-d83d8ff39dd4</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://demo-app.online/api/updateprofile</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
