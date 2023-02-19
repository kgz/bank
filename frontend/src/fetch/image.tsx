import { useContext, useEffect, useState } from "react";
import Cookies from 'universal-cookie';
import { userContext } from "..";


type ImageProps = {
    src: string;
    alt: string;
    key: number;
    noCahce?: boolean;
    [key: string]: any;
}

const Image = (props: ImageProps) => {
    const { src, alt, ...rest } = props;
    const [loading, setLoading] = useState(true);
    const { user, setUser } = useContext(userContext);
    const { profileUrl, profileCache } = user;

    const cookies = new Cookies();
    const [lastCache, setLastCache] = useState<any>();

    useEffect(() => {
        const controller = new AbortController();
        const signal = controller.signal;
        const token = cookies.get('token');
        // set same-origin 
        const headers = {
            // type image binanry   
            'Authorization': 'Bearer ' + token
        }

        if ((lastCache === profileCache) && user.profileUrl) {
            setLoading(false);
            return;
        }

        fetch('/me?' + user.profileCache, {
            method: 'GET',
            headers: headers,
            signal: signal
        })
            // image
            .then(res => res.blob())
            .then(blob => {
                const url = URL.createObjectURL(blob);
                setUser((prev: any) => {
                    return {
                        ...prev,
                        profileUrl: url
                    }
                })
                setLastCache(profileCache);

                setLoading(false);
            })
            .catch(err => {
                console.log(err);
                setLoading(false);
            })
            .finally(() => {
                setLoading(false);
            })

        return () => controller.abort();



    }, [user, profileCache, profileUrl, setUser, props.noCahce, cookies]);

    if (loading) return <div></div>;
    return <img src={profileUrl} alt={alt} {...rest}
    // set download link name

    />;
};


export default Image;