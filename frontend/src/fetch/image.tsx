import { useEffect, useState } from "react";
import Cookies from 'universal-cookie';

type ImageProps ={
    src: string;
    alt: string;
    key: number;
}

const Imagea = (props: ImageProps) => {
    const { src, alt, ...rest } = props;
    const [ url, setUrl ] = useState('');
    const [loading , setLoading] = useState(true);
    const cookies = new Cookies();

    useEffect(() => {
    
        const controller = new AbortController();
        const signal = controller.signal;
        const token = cookies.get('token');
   
        // set same-origin
        const headers = {
            // type image binanry
            
            'Authorization': 'Bearer ' + token
        }
        fetch('/me?' + new Date().getTime(), {
            method: 'GET',
            headers: headers,
            signal: signal
        })
            // image
            .then(res => res.blob())
            .then(blob => {
                const url = URL.createObjectURL(blob);
                console.log(url);
                setUrl(url);
                setLoading(false);

            })
            .catch(err => {
                console.log(err);
            })
            .finally(() => {
                setLoading(false);
            })

        return () => controller.abort();


    }, []);



    if (loading) return <div>loading...</div>;
    return <img src={url} alt={alt} {...rest}
        // set download link name
        
    />;
};


export default Imagea;