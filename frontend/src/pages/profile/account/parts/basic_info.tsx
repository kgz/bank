import React, { useContext } from 'react';
import { Button, Checkbox, Form, Input } from 'antd';
import { userContext } from '../../../..';

const onFinish = (values: any) => {
    console.log('Success:', values);
};

const onFinishFailed = (errorInfo: any) => {
    console.log('Failed:', errorInfo);
};

const testApi = () => {
    const controller = new AbortController();
    const signal = controller.signal;
    const headers = {
        'Content-Type': 'application/json',
    }
    fetch('/api/me/update', {
        method: 'POST',
        headers: headers,
        body: JSON.stringify({
            // username: "asdfasdf",
            email: "",
            validation_type: "userBasic"
        }),
        signal: signal
    })
        .then(res => {
            if (res.status === 200) {
                return res.json();
            }
            throw new Error(res.statusText);
        })
        .then(res => {
            console.log(res);
        })
        .catch(err => {
            console.log(err);
        })
}


const BasicInfo = () => {
    const {user, setUser} = useContext(userContext);

    return (
        <Form
            name="basic"
            size="large"
            labelCol={{ span: 8 }}
            wrapperCol={{ span: 16 }}
            fields={
                [
                    {
                        name: ['name'],
                        value: Math.random().toString(36).substring(7)
                    },
                    {
                        name: ['email'],
                        value: "test@test.com"
                    }
                ]
            }
            onFinish={onFinish}
            autoComplete="off"
        >

            <Button onClick={testApi}>Test</Button>
            <Form.Item
                name="name"
                rules={[{ required: true, message: 'Please input your username!' }]}
            >
                <Input addonBefore = "Display Name"/>
            </Form.Item>

            <Form.Item
                name="email"
                rules={[
                    {
                        type: 'email',
                        message: 'The input is not valid E-mail!',
                    },
                    {
                        required: true,
                        message: 'Please input your E-mail!',
                    },
                ]}
            >
                <Input addonBefore="Email"/>
            </Form.Item>

            <Form.Item name="remember" valuePropName="checked" wrapperCol={{ offset: 8, span: 16 }}>
                <Checkbox>Remember me</Checkbox>
            </Form.Item>

            <Form.Item wrapperCol={{ offset: 8, span: 16 }}>
                <Button type="primary" htmlType="submit">
                    Submit
                </Button>
            </Form.Item>
        </Form>
    )
}

export default BasicInfo;